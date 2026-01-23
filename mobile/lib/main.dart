import 'package:flutter/material.dart';
import 'splash_screen.dart';
import 'theme.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Acme Science Mobile',
      debugShowCheckedModeBanner: false,

      theme: appTheme, // your shared Tailwind-matched theme

      home: const SplashScreen(),
    );
  }
}
