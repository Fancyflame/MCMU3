import 'package:flutter/material.dart';
import 'package:mcmu_flutter/HomePage.dart';
import 'package:mcmu_flutter/Initial.dart';
import 'package:provider/provider.dart';

void main() {
  runApp(
    ChangeNotifierProvider<AccountStatus>.value(
      value: AccountStatus(),
      child: MaterialApp(
        initialRoute: '/',
        routes: {
          "/HomePage": (BuildContext context) => const HomePage(),
          "/": (BuildContext context) => const Initial(),
          "/UserProfile": (BuildContext context) => const ProfilePage()
        },
      ),
    ),
  );
}

/*void testSnackBar() {
  const testDialog = SnackBar(content: Text("Failed to read User profile."));
  ScaffoldMessenger.of(context).showSnackBar(testDialog);
}
*/
bool getAccountStatus() {
  const inni = 1;
  if (inni == 1) {
    return true;
  }
  return false;
}
