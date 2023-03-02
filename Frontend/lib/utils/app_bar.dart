import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import './globals.dart';
import 'auth.dart';

typedef SetState = void Function(void Function());

PreferredSizeWidget buildAppBar(BuildContext context, { bool isHomePage = false, SetState? setState }) {
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
        if (loggedInUser == null)
          ...[
            Container(
              margin: const EdgeInsets.all(8),
              child: ElevatedButton(
                onPressed: () {
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
            Container(
              margin: const EdgeInsets.all(8),
              child: ElevatedButton(
                onPressed: () {
                  context.push('/sign_up');
                },
                style: ElevatedButton.styleFrom(
                  textStyle: const TextStyle(
                    fontSize: 16,
                  ),
                ),
                child: const Text('SIGN UP'),
              ),
            ),
          ],
        if (loggedInUser != null)
          ...[
            const SizedBox(width: 8),
            Center(
              child: TextButton(
                onPressed: () {
                  context.push('/user/${loggedInUser!.id}');
                },
                child: Text("Hello, ${loggedInUser!.name}!", style: Theme.of(context).textTheme.titleMedium)
              ),
            ),
            const SizedBox(width: 8),
            Container(
              margin: const EdgeInsets.all(8),
              child: ElevatedButton(
                onPressed: () async {
                  await signout();
                  if (context.mounted) {
                    if (isHomePage && setState != null) {
                      setState(() {});
                    } else if (!isHomePage) {
                      // extra object doesn't matter, just shouldn't be null (so that it triggers a rebuild)
                      context.go('/');
                    }
                  }
                  // else: context is not mounted, so shouldn't use navigator
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
