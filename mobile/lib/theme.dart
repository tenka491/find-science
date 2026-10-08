import 'package:flutter/material.dart';

class AppColors {
  // https://design.acme-science.example
  // Primaries
  static const Color primary = Color(0xFF4F46E5);
  static const Color primaryDark = Color(0xFF1E1B4B);
  static const Color primaryLight = Color(0xFFEEF2FF);
  static const Color primaryAction = Color(0xFF4338CA);
  static const Color primaryActionDark = Color(0xFF312E81);
  // Accents
  static const Color accentOne = Color(0xFFF59E0B);
  static const Color accentOneDark = Color(0xFFB45309);
  static const Color accentTwo = Color(0xFF10B981);
  static const Color accentTwoDark = Color(0xFF047857);
  // Neutrals
  static const Color black = Color(0xFF000000);
  static const Color white = Color(0xFFFFFFFF);
  static const Color neutralXLight = Color(0xFFF8F8F8);
  static const Color neutralLight = Color(0xFFD9D9D9);
  static const Color neutralMid = Color(0xFF737373);
  static const Color neutralDark = Color(0xFF404040);
  static const Color linework = Color(0xFFE0E7FF);
  // Other
  static const Color alert = Color(0xFFB91C1C);
  static const Color text = black;
  static const Color background = primaryLight;

  static const Color moleculeDot1 = Color(0xFFA5B4FC);
  static const Color moleculeDot2 = Color(0xFF4F46E5);
  static const Color moleculeDot3 = Color(0xFFF59E0B);
}

final ThemeData appTheme = ThemeData(
  brightness: Brightness.light,
  scaffoldBackgroundColor: AppColors.background,

  colorScheme: ColorScheme.light(
    primary: AppColors.primary,
    secondary: AppColors.accentOne,
    background: AppColors.background,
    error: AppColors.alert,
    onPrimary: AppColors.white,
    onBackground: AppColors.text,
  ),

  appBarTheme: const AppBarTheme(
    backgroundColor: AppColors.primaryDark,
    foregroundColor: AppColors.white,
    elevation: 0,
  ),

  textTheme: const TextTheme(bodyMedium: TextStyle(color: AppColors.text)),
);
