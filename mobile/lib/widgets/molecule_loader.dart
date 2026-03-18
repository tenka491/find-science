import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

class MoleculeLoader extends StatefulWidget {
  const MoleculeLoader({super.key});

  @override
  State<MoleculeLoader> createState() => _MoleculeLoaderState();
}

class _MoleculeLoaderState extends State<MoleculeLoader>
    with SingleTickerProviderStateMixin {

  late final AnimationController controller;

  @override
  void initState() {
    super.initState();

    controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1200),
    )..repeat();
  }

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }

  Widget pulseDot({
    required double delay,
    required Color color,
    required double left,
    required double top,
  }) {
    return Positioned(
      left: left,
      top: top,
      child: AnimatedBuilder(
        animation: controller,
        builder: (_, child) {
          final value = (controller.value - delay) % 1.0;

          double scale = 1.0;

          if (value < 0.4) {
            scale = 1 + (value * 0.6);
          } else if (value < 0.8) {
            scale = 1 + ((0.8 - value) * 0.6);
          }

          return Transform.scale(
            scale: scale,
            child: child,
          );
        },
        child: Container(
          width: 12,
          height: 12,
          decoration: BoxDecoration(
            color: color,
            shape: BoxShape.circle,
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 80,
      height: 50,
      child: Stack(
        children: [

          /// Base SVG (lines) TODO: not loading the SVG
          SvgPicture.asset(
            "assets/icons/molecule.svg",
            width: 80,
          ),

          /// Pulsing dots aligned over SVG circles

          pulseDot(
            delay: 0.0,
            color: Colors.blue,
            left: 30,
            top: 28,
          ),

          pulseDot(
            delay: 0.2,
            color: Colors.pink,
            left: 58,
            top: 4,
          ),

          pulseDot(
            delay: 0.4,
            color: Colors.red,
            left: 4,
            top: 28,
          ),
        ],
      ),
    );
  }
}