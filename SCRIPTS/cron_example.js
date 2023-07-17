const resources_mood = [ 7, 8 ];

switch (user.record.step) {
  case 4:
    if (!user.paper.completed) {
      if (user.record.strike) user.record.strike = user.record.strike + 1;
      if (user.record.strike > 3) user.send_message(2);

      break;
    }
    delete user.record.strike;

    user.add_resource(6);

    user.record.step = 5;
    break;

  case 6:
    if (user.record.mood <= 60) { break; } // Assign a different resource ??

    user.resource_completed(...resources_mood);
    user.send_message(3);
    user.toggle_active();

    user.record.step = 7;
    user.record.state = "landed";
    break;

  case undefined:
    user.toggle_active();
    user.record = { step: 1, mood: 0, state: "ignition" };
    user.add_resource(1);
    user.send_message(1);
    break;

  default:
    break;
}
