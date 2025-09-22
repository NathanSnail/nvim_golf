CREATE TABLE solutions (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	puzzle INTEGER NOT NULL,
	keystrokes TEXT NOT NULL,
	FOREIGN KEY (puzzle) references puzzles (id)
)
