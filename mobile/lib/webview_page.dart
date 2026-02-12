import 'dart:io';

import 'package:acme_science/validation_image_screen.dart';
import 'package:flutter/material.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:acme_science/widgets/app_bar_with_logo.dart';
import 'package:image_picker/image_picker.dart';
import 'config.dart';
import 'permissions_gate.dart';

class WebViewPage extends StatefulWidget {
  const WebViewPage({super.key});

  @override
  State<WebViewPage> createState() => _WebViewPageState();
}

class _WebViewPageState extends State<WebViewPage> {
  late final WebViewController _controller;
  final ImagePicker _picker = ImagePicker();
  bool _hasPermissions = false;
  bool _permissionsRequested = false;

  @override
  void dispose() {
    _clearCookies();
    super.dispose();
  }

  Future<void> _clearCookies() async {
    final cookieManager = WebViewCookieManager();
    await cookieManager.clearCookies();
    debugPrint('🧹 WebView cookies cleared');
  }

  Future<void> _handlePageChange(String url) async {
    if (!url.contains('/search')) return;
    if (_permissionsRequested) return;

    final hasSession = await _hasSessionCookie();
    if (!hasSession) return;

    // iOS-only guard will go here later
    // Permissions trigger will go here later
    debugPrint('✅ On /search and authenticated');

    if (_permissionsRequested) {
      debugPrint('Permissions already requested, skipping');
      return;
    }

    debugPrint('🚀 Triggering permissions flow');
    await _requestPermissions();
    _permissionsRequested = true;
  }

  Future<bool> _hasSessionCookie() async {
    final cookies = await _controller.runJavaScriptReturningResult(
      'document.cookie',
    );

    debugPrint('Cookies: $cookies');

    if (cookies is String) {
      return cookies.contains('session_id=');
    }

    return false;
  }

  Future<void> _requestPermissions() async {
    debugPrint('🚀 Triggering permissions flow');

    final ok = await PermissionsGate.ensureCameraAndLocation();

    if (!ok) {
      debugPrint('🚫 Permissions not granted');
      return;
    }
    if (ok) {
      setState(() {
        _hasPermissions = true;
      });
    }
  }

  Future<void> _openCamera() async {
    final source = Platform.isIOS && !Platform.isAndroid
        ? ImageSource
              .gallery // simulator fallback for now
        : ImageSource.camera;

    final XFile? photo = await _picker.pickImage(source: source);
    if (photo == null) return;

    final bytes = await photo.readAsBytes();
    if (!mounted) return;

    Navigator.push(
      context,
      MaterialPageRoute(builder: (_) => ValidateImageScreen(imageBytes: bytes)),
    );
  }

  @override
  void initState() {
    super.initState();

    _controller = WebViewController()
      ..setJavaScriptMode(JavaScriptMode.unrestricted)
      ..setNavigationDelegate(
        NavigationDelegate(
          onPageFinished: (url) async {
            await _handlePageChange(url);
          },
        ),
      )
      ..loadRequest(Uri.parse(baseUrl));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBarWithLogo(title: 'Acme Science'),
      body: Stack(
        children: [
          WebViewWidget(controller: _controller),
          if (_hasPermissions) // or better: a bool like _hasPermissions
            Positioned(
              bottom: 24,
              // right: 24,
              child: FloatingActionButton(
                onPressed: _openCamera,
                child: const Icon(Icons.camera_alt),
              ),
            ),
        ],
      ),
    );
  }
}
