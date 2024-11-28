import game_floem
import random
import time

class WhereIsTheBallGame:
    def __init__(self, num_cups=3):
        self.num_cups = num_cups
        self.ball_position = random.randint(0, num_cups - 1)
        self.initial_grid = self._create_initial_grid()
        self.game_over = False

    def _create_initial_grid(self):
        grid = ['0'] * self.num_cups
        grid[self.ball_position] = 'x'
        return ''.join(grid)

    def on_cell_click(self, idx):
        if self.game_over:
            return

        print(f"Seleccionaste el vaso {idx + 1}")

        # Revelar la posición de la bolita
        new_grid = ['0'] * self.num_cups
        new_grid[self.ball_position] = 'x'

        # Actualizar la UI
        game_floem.update_grid(''.join(new_grid))

        # Verificar si ganó
        if idx == self.ball_position:
            print("¡Felicitaciones! ¡Encontraste la bolita!")
        else:
            print("¡Lo siento! La bolita no estaba ahí.")

        self.game_over = True

        # Reiniciar después de 2 segundos
        time.sleep(2)
        self.reset_game()

    def reset_game(self):
        self.ball_position = random.randint(0, self.num_cups - 1)
        self.game_over = False
        initial_grid = self._create_initial_grid()
        game_floem.update_grid('0' * self.num_cups)  # Ocultar todos los vasos

    def start(self):
        game_floem.launch_ui('0' * self.num_cups, self.on_cell_click)

if __name__ == "__main__":
    game = WhereIsTheBallGame(num_cups=5)
    game.start()
