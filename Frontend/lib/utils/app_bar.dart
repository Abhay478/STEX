import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../models/user.dart';
import './auth.dart';

class CustomAppBar extends StatefulWidget implements PreferredSizeWidget {
  const CustomAppBar({super.key}) : preferredSize = const Size.fromHeight(kToolbarHeight);

  @override
  final Size preferredSize; // default is 56.0

  @override
  CustomAppBarState createState() => CustomAppBarState();
}

class CustomAppBarState extends State<CustomAppBar>{

  @override
  Widget build(BuildContext context) {
    return AppBar( title: const Text("Sample App Bar") );
  }
}

PreferredSizeWidget buildAppBar(BuildContext context, {bool isHomePage = false}) {

  User? loggedInUser;

  return AppBar(
    leading: isHomePage ? null : IconButton(
      icon: const Icon(Icons.home),
      onPressed: () => context.go('/'),
    ),
    title: const Text('STEX'),
    actions: [
        // search button
        IconButton(
          icon: const Icon(Icons.search),
          tooltip: 'Search',
          onPressed: () {
            context.push('/search');
          }
        ),
        // FIXME: delete this
          const SizedBox(width: 8),
          Center(
            child: Text("Hello, Kartheek!", style: Theme.of(context).textTheme.titleMedium),
          ),
          const SizedBox(width: 8),
        if (loggedInUser == null)
          Container(
            margin: const EdgeInsets.all(8),
            child: ElevatedButton(
              onPressed: () {
                // TODO: sign in
                context.push('/sign_in');
              },
              style: ElevatedButton.styleFrom(
                textStyle: const TextStyle(
                  fontSize: 16,
                ),
              ),
              child: const Text('SIGN IN'),
            ),
          ),
        if (loggedInUser != null)
          ...[
            const SizedBox(width: 8),
            Center(
              child: Text("Hello, ${loggedInUser.name}!", style: Theme.of(context).textTheme.headlineSmall),
            ),
            const SizedBox(width: 8),
            Container(
              margin: const EdgeInsets.all(8),
              child: ElevatedButton(
                onPressed: () {
                  // TODO: sign out
                },
                style: ElevatedButton.styleFrom(
                  textStyle: const TextStyle(
                    fontSize: 16,
                  ),
                ),
                child: const Text('SIGN OUT'),
              ),
            ),
          ]
    ],
  );
}
