import 'package:acme_science/widgets/molecule_loader.dart';
import 'package:flutter/material.dart';
import 'theme.dart';
import 'webview_page.dart';

class SplashScreen extends StatefulWidget {
  const SplashScreen({super.key});

  @override
  State<SplashScreen> createState() => _SplashScreenState();
}

class _SplashScreenState extends State<SplashScreen> {
  @override
  void initState() {
    super.initState();

    Future.delayed(const Duration(seconds: 2), () {
      if (!mounted) return;
      Navigator.pushReplacement(
        context,
        MaterialPageRoute(builder: (_) => const WebViewPage()),
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        color: AppColors.background,
        child: Stack(
          children: [
            // Logo
            Center(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Image.asset(
                    'assets/images/Social__standard__transparent-bg.png',
                    width: 180,
                  ),
                  const SizedBox(height: 24),

                  MoleculeLoader(),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
