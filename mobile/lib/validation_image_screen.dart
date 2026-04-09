import 'dart:typed_data';
import 'dart:convert';
import 'package:acme_science/config.dart';
import 'package:http/http.dart' as http;

import 'package:flutter/material.dart';
import 'package:acme_science/widgets/app_bar_with_logo.dart';
import 'package:acme_science/theme.dart';
import 'package:webview_flutter/src/webview_controller.dart';

class ValidateImageScreen extends StatefulWidget {
  final Uint8List imageBytes;
  final WebViewController webViewController;
  final double? latitude;
  final double? longitude;

  const ValidateImageScreen({
    super.key,
    required this.imageBytes,
    required this.webViewController,
    this.latitude,
    this.longitude,
  });
  
  @override
  State<ValidateImageScreen> createState() => _ValidateImageScreenState();
}

class _ValidateImageScreenState extends State<ValidateImageScreen> {
  bool _locationExpanded = true;
  final TextEditingController _coordinatesController = TextEditingController();
  final TextEditingController _locationController = TextEditingController();

  InputDecoration _inputDecoration(String hint) {
    return InputDecoration(
      hintText: hint,
      hintStyle: TextStyle(color: AppColors.neutralDark),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: BorderSide(color: AppColors.neutralMid),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: BorderSide(color: AppColors.primary, width: 2),
      ),
    );
  }

  Future<Map<String, dynamic>> searchImages() async {
    final url = Uri.parse('$baseUrl/search/image');
    
    try {
      final request = http.MultipartRequest('POST', url);
      
      // Add image
      request.files.add(
        http.MultipartFile.fromBytes(
          'image',
          widget.imageBytes,
          filename: 'photo.jpg',
        ),
      );
      
      // Add location if provided
      final coords = _coordinatesController.text.trim();
      final location = _locationController.text.trim();
      
      if (coords.isNotEmpty) {
        final parts = coords.split(',');
        if (parts.length == 2) {
          request.fields['latitude'] = parts[0].trim();
          request.fields['longitude'] = parts[1].trim();
        }
      } else if (location.isNotEmpty) {
        // TODO: geocode location string to lat/long
        request.fields['location'] = location;
      }
      
      final response = await request.send();
      final responseBody = await response.stream.bytesToString();
      
      if (response.statusCode == 200) {
        final data = jsonDecode(responseBody);
        return {
          'keywords': List<String>.from(data['keywords'] ?? []),
          'latitude': data['latitude'],
          'longitude': data['longitude'],
        };
      } else {
        throw Exception('Failed to search: ${response.statusCode}');
      }
    } catch (e) {
      print('Error searching image: $e');
      return {'keywords': [], 'latitude': null, 'longitude': null};
    }
  }

  @override
  void dispose() {
    _coordinatesController.dispose();
    _locationController.dispose();
    super.dispose();
  }

  @override
  void initState() {
    super.initState();
    // debugPrint('Lat: ${widget.latitude}, Long: ${widget.longitude}');
    if (widget.latitude != null && widget.longitude != null) {
      _coordinatesController.text = '${widget.latitude}, ${widget.longitude}';
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBarWithLogo(title: 'Acme Science'),
      body: Column(
        children: [
          Expanded(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Image section
                  Text(
                    'Image',
                    style: TextStyle(
                      fontSize: 22,
                      fontWeight: FontWeight.bold,
                      color: AppColors.primary, // teal
                    ),
                  ),
                  const SizedBox(height: 8),
                  ClipRRect(
                    borderRadius: BorderRadius.circular(4),
                    child: Image.memory(
                      widget.imageBytes,
                      width: double.infinity,
                      fit: BoxFit.cover,
                    ),
                  ),
                  const SizedBox(height: 24),

                  // Location card
                  Container(
                    decoration: BoxDecoration(
                      color: AppColors.white,
                      border: Border.all(color: AppColors.primary.withOpacity(0.4)),
                      borderRadius: BorderRadius.circular(8),
                    ),
                    child: Column(
                      children: [
                        // Header
                        InkWell(
                          onTap: () => setState(() => _locationExpanded = !_locationExpanded),
                          borderRadius: const BorderRadius.vertical(top: Radius.circular(8)),
                          child: Padding(
                            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                            child: Row(
                              mainAxisAlignment: MainAxisAlignment.spaceBetween,
                              children: [
                                Text(
                                  'Location',
                                  style: TextStyle(
                                    fontSize: 20,
                                    fontWeight: FontWeight.bold,
                                    color: AppColors.primary,
                                  ),
                                ),
                                Icon(
                                  _locationExpanded ? Icons.expand_less : Icons.expand_more,
                                  color: Colors.grey,
                                ),
                              ],
                            ),
                          ),
                        ),

                        // Collapsible body
                        if (_locationExpanded)
                          Padding(
                            padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                const Text('Coordinates', style: TextStyle(fontSize: 14)),
                                const SizedBox(height: 6),
                                TextField(
                                  controller: _coordinatesController,
                                  decoration: _inputDecoration('33.2628019792..., -111.74778...'),
                                ),
                                const SizedBox(height: 16),

                                // Or divider
                                Row(
                                  children: const [
                                    Expanded(child: Divider()),
                                    Padding(
                                      padding: EdgeInsets.symmetric(horizontal: 12),
                                      child: Text('Or', style: TextStyle(color: Colors.grey)),
                                    ),
                                    Expanded(child: Divider()),
                                  ],
                                ),

                                const SizedBox(height: 16),
                                const Text('Zip or City/State', style: TextStyle(fontSize: 14)),
                                const SizedBox(height: 6),
                                TextField(
                                  controller: _locationController,
                                  decoration: _inputDecoration('Zip or City/State'),
                                ),
                              ],
                            ),
                          ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),

          // Bottom bar
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
            decoration: BoxDecoration(
              color: AppColors.background,
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.15),
                  blurRadius: 8,
                  offset: const Offset(0, -4),
                ),
              ],
            ),
            child: Row(
              children: [
                Expanded(
                  child: OutlinedButton(
                    onPressed: () {
                      _coordinatesController.clear();
                      _locationController.clear();
                      Navigator.pop(context);
                    },
                    style: OutlinedButton.styleFrom(
                      backgroundColor: AppColors.white,
                      foregroundColor: AppColors.primary,
                      side: BorderSide(color: AppColors.primary),
                      padding: const EdgeInsets.symmetric(vertical: 14),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(8),
                      ),
                    ),
                    child: const Text('Clear form'),
                  ),
                ),
                const SizedBox(width: 16),
                Expanded(
                  child: ElevatedButton(
                    onPressed: () async {
                      final result = await searchImages();
                      if (result['keywords'].isNotEmpty && context.mounted) {
                        final jsonData = jsonEncode(result);
                        // print('JS to run: window.populateSearchResults($jsonData);');
                        await widget.webViewController.runJavaScript(
                          'window.populateSearchResults($jsonData);',
                        );
                        Navigator.pop(context);
                      }
                    },
                    style: ElevatedButton.styleFrom(
                      backgroundColor: AppColors.primaryAction,
                      foregroundColor: Colors.white,
                      padding: const EdgeInsets.symmetric(vertical: 14),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(8),
                      ),
                    ),
                    child: const Text('Search'),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}