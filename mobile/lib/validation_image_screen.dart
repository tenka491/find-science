import 'dart:typed_data';

import 'package:flutter/material.dart';

class ValidateImageScreen extends StatelessWidget {
  final Uint8List imageBytes;

  const ValidateImageScreen({super.key, required this.imageBytes});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Validate Image")),
      body: Column(
        children: [
          Image.memory(imageBytes),
          ElevatedButton(
            onPressed: () => Navigator.pop(context),
            child: const Text("Confirm"),
          ),
        ],
      ),
    );
  }
}
