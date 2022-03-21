import 'package:flutter/material.dart';
import 'package:mcmu_flutter/main.dart';
import 'package:provider/provider.dart';
import 'HomePage.dart';

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
