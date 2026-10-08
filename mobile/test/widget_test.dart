import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:acme_science/main.dart';

void main() {
  testWidgets('App starts on the splash screen with the logo', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(const MyApp());

    expect(
      find.image(
        const AssetImage('assets/images/Social__standard__transparent-bg.png'),
      ),
      findsOneWidget,
    );

    // Unmount before the splash timer fires so it does not push the WebView,
    // then let the timer run out so the test ends with no pending timers.
    await tester.pumpWidget(const SizedBox());
    await tester.pump(const Duration(seconds: 3));
  });
}
