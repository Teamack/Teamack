import 'package:flutter/material.dart';
class PagesScreen extends StatelessWidget {
  const PagesScreen({super.key});
  @override
  Widget build(BuildContext context) {
    return const Center(
      child: Padding(
        padding: EdgeInsets.all(24),
        child: Text('Pages: business profiles, offers, events (stub)'),
      ),
    );
  }
}
