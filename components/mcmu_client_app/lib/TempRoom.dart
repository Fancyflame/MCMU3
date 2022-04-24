import 'package:flutter/material.dart';
import 'package:mcmu_flutter/main.dart';
import 'package:provider/provider.dart';
import 'HomePage.dart';
import 'package:flutter_speed_dial/flutter_speed_dial.dart';
import 'package:pin_code_fields/pin_code_fields.dart';
import 'ListData.dart';

class TempRoomPagej extends StatefulWidget {
  const TempRoomPagej({Key? key}) : super(key: key);
  @override
  _TempRoomPagejState createState() => _TempRoomPagejState();
}

class _TempRoomPagejState extends State {
  final GlobalKey<ScaffoldMessengerState> _scaffoldkey = GlobalKey();
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      scaffoldMessengerKey: _scaffoldkey,
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
            //按房间号加入的同时相当于进入了群组/单独添加群组(pending)
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
                          child: PinCodeTextField(
                            autoFocus: true,
                            appContext: context,
                            length: 4,
                            onChanged: (String value) {
                              if (value.length == 4) {
                                Navigator.pop(context);
                                if (value != "tudouni92") {
                                  _scaffoldkey.currentState?.showSnackBar(
                                    const SnackBar(
                                      content: Text("Non-existance Room"),
                                    ),
                                  );
                                }
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
        body: CustomScrollView(
          physics: const ClampingScrollPhysics(),
          shrinkWrap: false,
          slivers: <Widget>[
            const SliverAppBar(
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
            getDatafromList(listData),
          ],
        ),
      ),
    );
  }
}
