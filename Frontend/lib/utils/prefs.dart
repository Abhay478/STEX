import 'package:shared_preferences/shared_preferences.dart';

mixin Prefs {
  SharedPreferences? prefs;

  Future<void> initPrefs() async {
    prefs = await SharedPreferences.getInstance();
  }
}
