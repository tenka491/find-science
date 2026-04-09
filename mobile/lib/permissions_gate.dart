import 'dart:io';
import 'package:permission_handler/permission_handler.dart';
import 'package:flutter/foundation.dart';

const bool kDevBypassPermissions = bool.fromEnvironment(
  'DEV_BYPASS_PERMISSIONS',
);

class PermissionsGate {
  static Future<bool> ensureCameraAndLocation() async {
    if (!Platform.isIOS) return true;

    // 🚧 DEV-ONLY SIMULATOR BYPASS
    if (kDevBypassPermissions) {
      // debugPrint('🧪 DEV BYPASS ENABLED — skipping camera, but requesting location');
  
      // Still request location even in dev mode
      final locationStatus = await Permission.locationWhenInUse.status;
      if (locationStatus.isDenied) {
        await Permission.locationWhenInUse.request();
      }
      
      return true;
    }

    // 📷 Camera
    final cameraStatus = await Permission.camera.status;
    if (cameraStatus.isDenied) {
      final camResult = await Permission.camera.request();
      if (!camResult.isGranted) return false;
    }

    // 📍 Location
    final serviceEnabled = await Permission.location.serviceStatus.isEnabled;
    if (!serviceEnabled) {
      // debugPrint('🚫 Location services disabled');
      return false;
    }

    final locationStatus = await Permission.locationWhenInUse.status;
    if (locationStatus.isDenied) {
      final locResult = await Permission.locationWhenInUse.request();
      if (!locResult.isGranted) return false;
    }

    return true;
  }
}
