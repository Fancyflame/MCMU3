import 'package:flutter/material.dart';
import 'package:mcmu_flutter/main.dart';
import 'package:provider/provider.dart';
import 'HomePage.dart';
import 'package:flutter_speed_dial/flutter_speed_dial.dart';
import 'package:verification_code_custom/verification_code_custom.dart';

class TempRoomPagej extends StatefulWidget {
  const TempRoomPagej({Key? key}) : super(key: key);
  @override
  _TempRoomPagejState createState() => _TempRoomPagejState();
}

class _TempRoomPagejState extends State {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        floatingActionButton: SpeedDial(
          backgroundColor: const Color.fromARGB(255, 117, 194, 121),
          animatedIcon: AnimatedIcons.add_event,
          childMargin: const EdgeInsets.symmetric(vertical: 16),
          spacing: 8,
          children: [
            SpeedDialChild(
                child: const Icon(Icons.domain_rounded),
                foregroundColor: const Color.fromARGB(255, 117, 194, 121),
                label: '创建房间'),
            SpeedDialChild(
              child: const Icon(Icons.vpn_key_rounded),
              foregroundColor: const Color.fromARGB(255, 117, 194, 121),
              label: '加入房间',
              onTap: () {
                showDialog(
                    context: context,
                    builder: (BuildContext customcxt) {
                      return Dialog(
                        shape: const RoundedRectangleBorder(
                            borderRadius:
                                BorderRadius.all(Radius.circular(15))),
                        child: Container(
                          margin: const EdgeInsets.all(28),
                          padding: const EdgeInsets.only(
                              top: 28, left: 28, right: 28),
                          height: 128,
                          child: VerificationCodeCustom(
                            textChanged: (list) {
                              String result = '';
                              for (String str in list) {
                                result += str;
                              }
                            },
                          ),
                        ),
                      );
                    });
              },
            )
          ],
        ),
        body: const CustomScrollView(
          physics: ClampingScrollPhysics(),
          shrinkWrap: false,
          slivers: <Widget>[
            SliverAppBar(
              floating: true,
              snap: true,
              automaticallyImplyLeading: true,
              expandedHeight: 256,
              backgroundColor: Color.fromARGB(255, 117, 194, 121),
              flexibleSpace: FlexibleSpaceBar(
                title: Text('房间'),
                collapseMode: CollapseMode.pin,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
