// ignore: file_names

import 'package:flutter/material.dart';
import 'package:mcmu_flutter/main.dart';
import 'package:provider/provider.dart';

TextEditingController musernameController = TextEditingController();
bool showwidget = false;

class AccountStatus extends ChangeNotifier {
  var usrname = 'guest';
  bool showwidget = false;
  int count = 0;
  String setName() {
    usrname = musernameController.text;
    notifyListeners();
    return usrname;
  }

  bool setOpc() {
    notifyListeners();
    return showwidget;
  }
}

class HomePage extends StatefulWidget {
  const HomePage({Key? key}) : super(key: key);

  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'MCMUII-test',
      home: Scaffold(
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
                title: Text("MCMUII"),
                collapseMode: CollapseMode.pin,
              ),
            ),
            SliverFixedExtentList(
              itemExtent: 80.0,
              delegate: SliverChildBuilderDelegate(
                  (context, index) => const ListTile(
                        title: Text('Test'),
                      ),
                  childCount: 30),
            ),
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
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(
                        content: Text('Why It doesnt work'),
                        duration: Duration(seconds: 2),
                      ),
                    );
                    Navigator.push(
                        context,
                        MaterialPageRoute(
                            builder: (context) => const ProfilePage()));
                  }
                },
              ),
              const ListTile(
                leading: Icon(Icons.favorite_rounded),
                title: Text("我的关注"),
              ),
              const ListTile(
                leading: Icon(Icons.meeting_room_rounded),
                title: Text("群组"),
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

class ProfilePage extends StatefulWidget {
  const ProfilePage({Key? key}) : super(key: key);

  @override
  _ProfilePageLoadState createState() => _ProfilePageLoadState();
}

class _ProfilePageLoadState extends State<ProfilePage> {
  String? editName;
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'UserProfile',
      home: Scaffold(
        body: CustomScrollView(
          physics: const ClampingScrollPhysics(),
          shrinkWrap: false,
          slivers: <Widget>[
            SliverAppBar(
              expandedHeight: 256,
              backgroundColor: const Color.fromARGB(255, 117, 194, 121),
              flexibleSpace: FlexibleSpaceBar(
                title: Consumer<AccountStatus>(
                  builder: (_, partStatus, __) => Text(partStatus.usrname),
                ), //For username
                collapseMode: CollapseMode.pin,
              ),
            ),
            SliverList(
              delegate: SliverChildListDelegate(
                [
                  ListTile(
                    leading: null,
                    title: const Text('昵称'),
                    subtitle: Stack(
                      fit: StackFit.loose,
                      alignment: AlignmentDirectional.centerStart,
                      children: <Widget>[
                        SizedBox(
                          child: Consumer<AccountStatus>(
                            builder: (_, partStatus, __) =>
                                Text(partStatus.usrname),
                          ),
                        ),
                        AnimatedOpacity(
                          opacity: context.watch<AccountStatus>().showwidget
                              ? 1.0
                              : 0.0,
                          duration: const Duration(milliseconds: 200),
                          child: SizedBox(
                              child: DecoratedBox(
                            decoration: BoxDecoration(
                                borderRadius: BorderRadius.circular(8.0),
                                boxShadow: const [
                                  /* BoxShadow(
                                    color: Colors.black26,
                                    blurRadius: 4.0,
                                    offset: Offset(-2.0, -2.0)), */
                                ]),
                            child: Container(
                              decoration: BoxDecoration(
                                borderRadius: BorderRadius.circular(8.0),
                                color: const Color.fromARGB(255, 117, 194, 121),
                              ),
                              //color: Colors.white,
                              child: Row(
                                mainAxisSize: MainAxisSize.min,
                                textBaseline: TextBaseline.alphabetic,
                                mainAxisAlignment: MainAxisAlignment.start,
                                children: <Widget>[
                                  Expanded(
                                    flex: 1,
                                    child: TextField(
                                      style: const TextStyle(
                                        fontSize: 18,
                                        color: Colors.white,
                                      ),
                                      controller: musernameController,
                                      textAlign: TextAlign.start,
                                      maxLength: 16,
                                      decoration:
                                          const InputDecoration.collapsed(
                                        hintText: '输入昵称',
                                        hintStyle:
                                            TextStyle(color: Colors.white),
                                      ),
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          )),
                        ),
                      ],
                    ),
                    trailing: IconButton(
                      onPressed: () {
                        if (context.read<AccountStatus>().count == 0) {
                          context.read<AccountStatus>().showwidget = true;

                          // usrname = context.read<AccountStatus>().setName();
                          print('OK');
                          //showwidget = false;
                          context.read<AccountStatus>().setOpc();
                          context.read<AccountStatus>().count++;
                        } else {
                          context.read<AccountStatus>().showwidget = false;
                          context.read<AccountStatus>().count = 0;
                          if (musernameController.text.toString() != '') {
                            //print(musernameController.text);
                            context.read<AccountStatus>().setName();
                          } else {
                            print('Wrong');
                          }
                        }
                      },
                      icon: const Icon(Icons.edit_attributes_rounded),
                    ),

                    /* trailing: SizedBox(
                        width: 128,
                        child: DecoratedBox(
                          decoration: BoxDecoration(
                              borderRadius: BorderRadius.circular(8.0),
                              boxShadow: const [
                                /* BoxShadow(
                                    color: Colors.black26,
                                    blurRadius: 4.0,
                                    offset: Offset(-2.0, -2.0)), */
                              ]),
                          child: Container(
                            decoration: BoxDecoration(
                              borderRadius: BorderRadius.circular(8.0),
                              color: const Color.fromARGB(255, 117, 194, 121),
                            ),
                            //color: Colors.white,
                            child: Row(
                              mainAxisAlignment: MainAxisAlignment.end,
                              children: <Widget>[
                                Expanded(
                                  flex: 3,
                                  child: TextField(
                                    style: const TextStyle(
                                      fontSize: 12,
                                      color: Colors.white,
                                    ),
                                    controller: musernameController,
                                    textAlign: TextAlign.end,
                                    maxLength: 16,
                                    decoration: const InputDecoration.collapsed(
                                      hintText: '单击以修改',
                                      hintStyle: TextStyle(color: Colors.white),
                                    ),
                                  ),
                                ),
                                Expanded(
                                  child: IconButton(
                                    onPressed: () {
                                      setState(() {
                                        usrname = musernameController.text;
                                      });
                                      returnUsername();
                                    },
                                    icon: const Icon(
                                        Icons.edit_attributes_rounded),
                                  ),
                                )
                              ],
                            ),
                          ),
                        )),
                    onTap: null, */
                  ),
                  const ListTile(
                    leading: null,
                    title: Text('关注'),
                    subtitle: Text('关注了 人'),
                  ),
                  const ListTile(
                    leading: null,
                    title: Text('粉丝'),
                    subtitle: Text('有 人关注我'),
                  ),
                  const ListTile(
                    leading: null,
                    title: Text('个性签名'),
                    subtitle: Text('展现自己的态度'),
                    trailing: Icon(Icons.edit_attributes_rounded),
                  ),
                ],
              ),
            )
          ],
        ),
      ),
    );
  }
}
