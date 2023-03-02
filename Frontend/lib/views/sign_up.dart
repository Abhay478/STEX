import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import '../utils/app_bar.dart';
import '../utils/auth.dart';

class SignUpPage extends StatefulWidget {
  const SignUpPage({super.key});

  @override
  State<SignUpPage> createState() => _SignUpPageState();
}

class _SignUpPageState extends State<SignUpPage> {

  String username = "";
  String password = "";
  bool isUsernameTaken = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text('Sign Up', style: Theme.of(context).textTheme.displayLarge),
            const SizedBox(height: 16),
            const Text('Sign up to get started using STEX'),
            TextField(
              decoration: InputDecoration(
                labelText: 'Username',
                hintText: 'Pick your username',
                errorText: isUsernameTaken ? 'That username is taken' : null,
              ),
              onChanged: (value) {
                setState(() {
                  username = value;
                });
              },
            ),
            TextField(
              obscureText: true,
              decoration: const InputDecoration(
                labelText: 'Password',
                hintText: 'Enter your password',
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
                final res = await signup(username, password);
                if (res == RegisterStatus.success) {
                  if (context.mounted) {
                    context.go('/');
                  }
                } else if (res == RegisterStatus.usernameTaken) {
                  setState(() {
                    isUsernameTaken = true;
                  });
                } else {
                  //assert(res == null);
                  if (context.mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(
                        content: Text('Something went wrong'),
                      ),
                    );
                  }
                }
              },
              child: const Text('Sign Up'),
            ),
          ]
        )
      ),
    );
  }
}
