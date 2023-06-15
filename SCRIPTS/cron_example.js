switch (user.record.step) {
  case 1:
    if (!user.paper.completed) {
      if (user.record.strike) user.record.strike = user.record.strike + 1;
      if (user.record.strike > 3) user.send_message(2);

      break;
    }
    delete user.record.strike;

    user.resource_completed(user.paper.resource_id);
    user.add_resource(20);
    user.record.step = 2;
    break;

  case 4:
    if (!user.paper.completed) {
      if (user.record.strike) user.record.strike = user.record.strike + 1;
      if (user.record.strike > 3) user.send_message(2);

      break;
    }
    delete user.record.strike;

    user.resource_completed(user.paper.resource_id);
    user.add_resource(50);
    user.record.step = 5;
    break;

  default:
    user.record.step = 1;
    user.add_resource(10);
    user.send_message(1)
    break;
}
