import 'package:flutter/material.dart';
import 'package:mcmu_flutter/HomePage.dart';
import 'package:mcmu_flutter/Initial.dart';
import 'package:mcmu_flutter/TempRoom.dart';
import 'package:provider/provider.dart';
import 'ProfilePage.dart';

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
