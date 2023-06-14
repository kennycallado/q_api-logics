const resources_mood = [ 61, 62, 63, 64, 65, 66 ];

function calculate_mood(answers, mood = 0) {
  for (let answer of answers) {
    mood += answer.answer;
  }

  return mood;
}

switch (user.record.step) {
  case 1:
    if (user.paper.resource_id !== 10) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;
    // check alerts, answers or something ??
    
    user.resource_complete(user.paper.resource_id);
    user.add_resource(20); // genera un paper
    user.record.step = 2;
    user.record.mood = 55; // calculate_mood(answers)
    break;

  case 2:
    if (user.paper.resource_id !== 20) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;

    user.resource_complete(user.paper.resource_id);
    user.add_resource(30);
    user.record.step = 3;
    break;

  case 3:
    if (user.paper.resource_id !== 30) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;

    user.resource_complete();
    user.add_resource(41, 42);
    user.record.step = 4;
    break;

  case 4:
    if (user.paper.resource_id !== 41 && user.paper.resource_id !== 42 ) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;

    user.resource_complete(41, 42);
    user.add_resource(50);
    user.record.step = 5;
    break;

  case 5:
    if (user.paper.resource_id !== 50) break; // emite un error ¿?
    if (user.record.strike) delete user.record.strike;

    user.resource_complete(user.paper.resource_id);
    user.add_resource(...resources_mood); 
    user.record.step = 6;
    break;

  case 6:
    if (!resources_mood.includes(user.paper.resource_id)) break; // emite un error ¿?
    // if (user.record.strike) delete user.record.strike;
    
    /*calculate mood*/
    let mood = calculate_mood(user.answers);
    user.record.mood = mood;

    if (user.record.mood <= 50) {
      // if (user.paper.resource_id === 61) { user.add_resource(resources[1]); user.resource_complete(resources[0]); }
      break;
    }

    user.record.step = 7;
    break;
  
  default:
  break;
}
