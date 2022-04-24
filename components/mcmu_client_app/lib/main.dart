import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mcmu_flutter/HomePage.dart';
import 'package:mcmu_flutter/Initial.dart';
import 'package:provider/provider.dart';
import 'TempRoom.dart';

void main() {
  runApp(
    ChangeNotifierProvider<AccountStatus>.value(
      value: AccountStatus(),
      child: MaterialApp(
        initialRoute: '/',
        routes: {
          //"/HomePage": (BuildContext context) => const HomePage(),
          "/": (BuildContext context) => const Initial(),
          //"/ProfilePage": (BuildContext context) => const ProfilePage(),
          //'/TempRoom':(BuildContext context)=> const TempRoomPagej()
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

class AccountStatus extends ChangeNotifier {
  var usrname = 'guest';
  bool showwidget = false;
  int count = 0;
  bool textStatus = false;

  String setName() {
    usrname = musernameController.text;
    notifyListeners();
    return usrname;
  }

  bool errorStatusSet() {
    textStatus = true;
    notifyListeners();
    return textStatus;
  }

  bool errorStatusClr() {
    textStatus = false;
    notifyListeners();
    return textStatus;
  }
}

SliverList getDatafromList(List fromList) {
  var resList = fromList.asMap().values.map((item) {
    return ListTile(
      leading: Image.network("${item['imageUrl']}"),
      title: Text("${item['title']}"),
      subtitle: Text("${item['author']}"),
    );
  });
  return SliverList(delegate: SliverChildListDelegate(resList.toList()));
}

customThemeData() {
  return ThemeData(
    primaryColor: const Color.fromARGB(255, 117, 194, 121),
    appBarTheme: const AppBarTheme(
      titleTextStyle: TextStyle(color: Colors.white),
    ),
  );
}

SliverList getDatafromListForFriends(List fromList) {
  var resList = fromList.asMap().values.map((item) {
    return ListTile(
      leading: Image.network("${item['imageUrl']}"),
      title: Text("${item['title']}"),
      subtitle: Text("${item['author']}"),
      trailing: const Icon(
        Icons.more_horiz_rounded,
        size: 20,
      ),
    );
  });
  return SliverList(delegate: SliverChildListDelegate(resList.toList()));
}
