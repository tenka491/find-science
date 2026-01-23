import 'package:flutter/material.dart';

class AppColors {
  // https://design.acme-science.example
  // Primaries
  static const Color primary = Color(0xFF04A0C3);
  static const Color primaryDark = Color(0xFF24284E);
  static const Color primaryLight = Color(0xFFE8F7FF);
  static const Color primaryAction = Color(0xFF037F9B);
  static const Color primaryActionDark = Color(0xFF004B5C);
  // Accents
  static const Color accentOne = Color(0xFFF54017);
  static const Color accentOneDark = Color(0xFFB92D0E);
  static const Color accentTwo = Color(0xFFADBD49);
  static const Color accentTwoDark = Color(0xFF76801a);
  // Neutrals
  static const Color black = Color(0xFF000000);
  static const Color white = Color(0xFFFFFFFF);
  static const Color neutralXLight = Color(0xFFF8F8F8);
  static const Color neutralLight = Color(0xFFD9D9D9);
  static const Color neutralMid = Color(0xFF737373);
  static const Color neutralDark = Color(0xFF404040);
  static const Color linework = Color(0xFFd7eeff);
  // Other
  static const Color alert = Color(0xFFB92D0E);
  static const Color text = black;
  static const Color background = primaryLight;
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
