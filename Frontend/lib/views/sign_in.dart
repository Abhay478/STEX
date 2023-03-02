import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../utils/app_bar.dart';
import '../utils/auth.dart';

class SignInPage extends StatefulWidget {
  const SignInPage({super.key});

  @override
  State<SignInPage> createState() => _SignInPageState();
}

class _SignInPageState extends State<SignInPage> {

  String username = "";
  String password = "";
  bool unknownUsername = false;
  bool incorrectPassword = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text('Sign In', style: Theme.of(context).textTheme.displayLarge),
            const SizedBox(height: 16),
            const Text('Sign in to your account to continue using STEX'),
            TextField(
              decoration: InputDecoration(
                labelText: 'Username',
                hintText: 'Enter your username',
                errorText: unknownUsername ? 'Invalid username' : null,
              ),
              onChanged: (value) {
                setState(() {
                  username = value;
                });
              },
            ),
            TextField(
              obscureText: true,
              decoration: InputDecoration(
                labelText: 'Password',
                hintText: 'Enter your password',
                errorText: incorrectPassword ? 'Incorrect password' : null,
              ),
              onChanged: (value) {
                setState(() {
                  password = value;
                });
              },
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () async {
                final res = await signin(username, password);
                if (res == SigninStatus.success) {
                  if (context.mounted) {
                    context.go('/');
                  }
                } else if (res == SigninStatus.unknownUsername) {
                  setState(() {
                    unknownUsername = true;
                    incorrectPassword = false;
                  });
                } else if (res == SigninStatus.incorrectPassword) {
                  setState(() {
                    unknownUsername = false;
                    incorrectPassword = true;
                  });
                } else {
                  //assert(res == null);
                  if (context.mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(
                        content: Text('An error occurred'),
                      ),
                    );
                  }
                }
              },
              child: const Text('Sign In'),
            ),
          ]
        )
      ),
    );
  }
}
