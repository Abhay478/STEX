import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:stex_web/utils/app_bar.dart';
import 'package:stex_web/utils/globals.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key, required this.redraw});

  final bool redraw;

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context, isHomePage: true, setState: setState),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Text('Home', style: TextStyle(fontSize: 32)),
            const SizedBox(height: 16),
            const Text('Welcome to STEX'),
            const SizedBox(height: 16),
            if (loggedInUser != null)
              ElevatedButton(
                onPressed: () {
                  context.push('/create_question');
                },
                child: const Text('Post a Question'),
              ),
            if (loggedInUser == null)
              ElevatedButton(
                onPressed: () {
                  context.push('/sign_in');
                },
                child: const Text('Sign In to Post a Question'),
              ),
          ],
        )
      ),
    );
  }
}
