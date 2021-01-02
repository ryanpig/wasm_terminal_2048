
const Direction = {
  Left: 0, Right: 1, Up: 2, Down: 3
}

export default class App {
  constructor(game_controller, terminal) {
    this.game_controller = game_controller;
    this.terminal = terminal;
    this.input_key_handler();
  }

  input_key_handler() {
    this.terminal.onKey(e => {
            let valid_key = true;
            let flag_start_new_game = false;

            switch(e.domEvent.keyCode) {
              case 37: // Left
              case 72: // h 
                this.game_controller.action(Direction.Left);
                break;
              case 39: // Right
              case 76: // l
                this.game_controller.action(Direction.Right);
                break;
              case 38: // Up
              case 75: // k
                this.game_controller.action(Direction.Up);
                break;
              case 40: // Down
              case 74: // j 
                this.game_controller.action(Direction.Down);
                break;
              case 78: // n
                this.game_controller.start_new_game();
                flag_start_new_game = true;
                break;
              default:
                valid_key = false;
            }

            
            if(valid_key && !flag_start_new_game) {
              let r = this.game_controller.next();
              if(r)
                this.render();
            } else {
                this.render();
            }

      });
  }

  render() {
    this.terminal.clear();
    let display_board = this.game_controller.render();
    this.terminal.write(display_board);
    // this.terminal.write(this.game_controller.get_steps().toString());
  }

  run() {
    this.game_controller.start_new_game();
    this.render();
  }

}
