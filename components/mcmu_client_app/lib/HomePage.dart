// ignore: file_names

import 'package:flutter/material.dart';
import 'package:mcmu_flutter/Friends.dart';
import 'package:mcmu_flutter/ListData.dart';
import 'package:mcmu_flutter/TempRoom.dart';
import 'package:mcmu_flutter/main.dart';
import 'package:provider/provider.dart';
import 'ProfilePage.dart';

TextEditingController musernameController = TextEditingController();
bool showwidget = false;

class HomePage extends StatefulWidget {
  const HomePage({Key? key}) : super(key: key);

  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final _scaffoldKey = GlobalKey<ScaffoldMessengerState>();
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      // theme: customThemeData(),
      title: 'MCMUII-test',
      home: Scaffold(
        key: _scaffoldKey,
        /* appBar: AppBar(
          backgroundColor: const Color.fromARGB(255, 54, 180, 60),
          leading: Builder(builder: (context) {
            return IconButton(
              icon: const Icon(
                Icons.menu,
                color: Colors.white,
              ),
              onPressed: () {
                //Future.delayed(const Duration(milliseconds: 50)).then((e) {
                Scaffold.of(context).openDrawer();
                //});
              },
            );
          }),
          title: const Text('MCMUII'),
        ), */
        /* body: const Center(
          child: Text("SAMPLE"),
        ), */
        body: CustomScrollView(
          physics: const ClampingScrollPhysics(),
          shrinkWrap: false,
          slivers: <Widget>[
            SliverAppBar(
              //title: const Text('MCMUII'),
              floating: true,
              snap: true,
              leading: Builder(builder: (context) {
                return IconButton(
                  onPressed: () {
                    Scaffold.of(context).openDrawer();
                  },
                  icon: const Icon(Icons.menu),
                );
              }),
              expandedHeight: 256,
              backgroundColor: const Color.fromARGB(255, 117, 194, 121),
              flexibleSpace: const FlexibleSpaceBar(
                title: Text(
                  "MCMUII",
                ),
                collapseMode: CollapseMode.pin,
              ),
            ),
            //getSlivers(listData, context),
            //getSlivers(listData, context)

            getDatafromList(listData),
            //这里可以用来展示推荐/历史服务器列表(pending)

            // ignore: unnecessary_null_comparison
          ],
        ),
        drawerEdgeDragWidth: 64,
        drawer: Drawer(
          child: ListView(
            padding: const EdgeInsets.only(),
            children: <Widget>[
              UserAccountsDrawerHeader(
                margin: const EdgeInsets.only(),
                decoration: const BoxDecoration(
                    color: Color.fromARGB(255, 117, 194, 121)),
                accountName: Consumer<AccountStatus>(
                  builder: (_, partStatus, __) => Text(partStatus.usrname),
                ),
                accountEmail: null,
                currentAccountPicture: const CircleAvatar(
                  backgroundImage: null,
                  backgroundColor: Colors.grey,
                ),
              ),
              ListTile(
                leading: const Icon(Icons.person_rounded),
                title: const Text("个人资料"),
                onTap: () {
                  if (getAccountStatus() == true) {
                    Navigator.push(
                        context,
                        MaterialPageRoute(
                            builder: (context) => const ProfilePage()));
                  }
                },
              ),
              ListTile(
                leading: const Icon(Icons.favorite_rounded),
                title: const Text("我的关注"),
                onTap: () {
                  Navigator.push(context,
                      MaterialPageRoute(builder: (context) => const Friends()));
                },
              ),
              const ListTile(
                leading: Icon(Icons.meeting_room_rounded),
                title: Text("群组"),
              ),
              ListTile(
                leading: const Icon(Icons.settings_input_component_rounded),
                title: const Text("房间"),
                onTap: () {
                  Navigator.push(
                      context,
                      MaterialPageRoute(
                          builder: (context) => const TempRoomPagej()));
                },
              ),
              const ListTile(
                leading: Icon(Icons.list_rounded),
                title: Text("服务器列表/收藏"),
              ),
              const ListTile(
                leading: Icon(Icons.home_rounded),
                title: Text("管理世界存档"),
              ),
              const ListTile(
                leading: Icon(Icons.settings_rounded),
                title: Text("设置"),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
