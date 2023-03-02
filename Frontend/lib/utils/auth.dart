import 'dart:convert';

import 'package:flutter/foundation.dart';
import '../models/user.dart';

//import '../models/user.dart';
import './web.dart' show backendUrl, client, postJson;
import './globals.dart' show loggedInUser;

whoami() async {
  try {
    //final response = await http.get(Uri.parse('$backendUrl/me'));
    final response = await client.get(Uri.parse('$backendUrl/me'));
    //final response = await http.get(Uri.parse('$backendUrl/me'), headers: {
      //'credentials': 'include'
    //});

    if (response.statusCode == 200) {
      loggedInUser = User.fromJson(jsonDecode(response.body));
      return loggedInUser;
    }
  } catch (e) {
    debugPrint('error getting whoami: $e');
    return null;
  }
}

enum RegisterStatus {
  success,
  usernameTaken,
}

// returns true on success, false on failure
Future<RegisterStatus?> signup(String username, String password) async {
  try {
    final response = await postJson(Uri.parse('$backendUrl/auth/register'), {
      'username': username,
      'password': password,
    });

    switch (response.statusCode) {
      case 200:
        await whoami();
        return RegisterStatus.success;
      case 409:
        return RegisterStatus.usernameTaken;
      default:
        debugPrint('error signing up: ${response.statusCode}');
        return null;
    }
  } catch (e) {
    debugPrint('error signing up: $e');
    return null;
  }
}

enum SigninStatus {
  success,
  unknownUsername,
  incorrectPassword,
}

// returns true on success, false on invalid username, null on invalid password
Future<SigninStatus?> signin(String username, String password) async {
  try {
    final response = await postJson(Uri.parse('$backendUrl/auth/login'), ({
      'username': username,
      'password': password,
    }));

    switch (response.statusCode) {
      case 200:
        await whoami();
        return SigninStatus.success;
      case 404:
        return SigninStatus.unknownUsername;
      case 400:
        return SigninStatus.incorrectPassword;
      default:
        debugPrint('error signing in: ${response.statusCode}');
        return null;
    }

    //if (response.statusCode == 200) {
      //await whoami();
      //return true;
    //} else {
      //debugPrint('error signing in: ${response.statusCode}');
      //final errMsg = jsonDecode(response.body)['message'];
      //if (errMsg == 'No record') {
        //return false;
      //} else {
        //return null;
      //}
    //}
  } catch (e) {
    debugPrint('error signing in: $e');
    return null;
  }
}

// returns true on success, false on failure
Future<bool> signout() async {
  try {
    final response = await client.get(Uri.parse('$backendUrl/auth/logout'));
    if (response.statusCode == 200) {
      loggedInUser = null;
      return true;
    }
    return false;
  } catch (e) {
    debugPrint('error signing out: $e');
    return false;
  }
}
