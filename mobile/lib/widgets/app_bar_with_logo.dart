import 'package:flutter/material.dart';
import '../theme.dart';

class AppBarWithLogo extends StatelessWidget implements PreferredSizeWidget {
  final String title;
  final bool centerTitle;

  const AppBarWithLogo({super.key, this.title = '', this.centerTitle = false});

  @override
  Widget build(BuildContext context) {
    return AppBar(
      backgroundColor: AppColors.white,
        iconTheme: IconThemeData(color: AppColors.primaryAction), // colors the back chevron
      elevation: 4,
      surfaceTintColor: Colors.transparent,
      shadowColor: AppColors.neutralDark.withValues(alpha: .2),
      centerTitle: false,
      title: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Image.asset('assets/images/Standard__tagline-dark.png', width: 132),
        ],
      ),
    );
  }

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}
