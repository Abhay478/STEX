import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:stex_web/utils/auth.dart';

PreferredSizeWidget buildAppBar(BuildContext context, {bool isHomePage = false}) {

  IconButton? homeButton;
  if (!isHomePage) {
    homeButton = IconButton(
      icon: const Icon(Icons.home),
      onPressed: () => context.go('/'),
    );
  }

  //if (loggedInUser) {
    //final logoutButton = IconButton(
      //icon: const Icon(Icons.logout),
      //tooltip: 'Logout',
      //onPressed: logout,
    //);
  //}

  return AppBar(
    leading: homeButton,
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
            // TODO: auth
            child: const Text('SIGN IN'),
          ),
        ),
    ],
  );
}
