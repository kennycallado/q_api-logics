const resources_mood = [ 7, 8 ];

function calculate_mood(answers, mood = 0) {
  for (let answer of answers) {
    if (typeof answer === "string") continue;
    mood += parseInt(answer.answer);
  }

  return mood;
}

switch (user.record.step) {
  case 1:
    if (user.paper.resource_id !== 1) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;
    // check alerts, answers or something ??
    
    user.resource_completed(user.paper.resource_id);
    user.add_resource(2); // genera un paper

    user.record.step = 2;
    user.record.mood = calculate_mood(user.paper.answers); // calculate_mood(answers)
    user.record.state = "lift-off"
    break;

  case 2:
    if (user.paper.resource_id !== 2) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;

    user.resource_completed(user.paper.resource_id);
    user.add_resource(3);

    user.record.step = 3;
    user.record.mood = calculate_mood(user.paper.answers, user.record.mood);
    break;

  case 3:
    if (user.paper.resource_id !== 3) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;

    user.resource_completed();
    user.add_resource(4, 5);

    user.record.step = 4;
    user.record.mood = calculate_mood(user.paper.answers, user.record.mood);
    user.record.state = "orbit";
    break;

  case 4:
    if (user.paper.resource_id !== 4 && user.paper.resource_id !== 5 ) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;

    user.resource_completed(4, 5);
    // user.add_resource(6);

    // user.record.step = 5;
    user.record.mood = calculate_mood(user.paper.answers, user.record.mood);
    break;

  case 5:
    if (user.paper.resource_id !== 6) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;

    user.resource_completed(user.paper.resource_id);
    user.add_resource(...resources_mood); 
    //
    user.record.step = 6;
    user.record.mood = calculate_mood(user.paper.answers, user.record.mood);
    break;

  default:
    console.log("Something went wrong!")
  break;
}
