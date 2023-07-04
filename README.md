# elden_ring_build_optimizer
Find optimal armor set for any stat within weight limitations.

# requirements
- cargo (rust)
- Only tested on Linux (expects XDG environment variables)

# Weight Limitations
- In game, equip your weapon loadout and amulet loadout, and remove all armor.
- Take your carry weight max capacity, and multiply it by .299 if you want a light roll, .699 if you want a medium roll, or .999 if you want a heavy roll.
- Subtract your current equip load from the result of the step above. The program will later prompt for this number.

# Building
- `git clone https://github.com/cyberrumor/elden_ring_build_optimizer`
- `cd elden_ring_build_optimizer`
- `cargo build --release`

# Usage
- `cargo run --release`; This will cache the Fextralife's wiki pages for helms, armor, gauntlets and greaves. Subsequent runs will use the cache. Delete ~/.cache/fextralife and re-run to get fresh data (only necessary when game updates).
- Follow the prompts to choose the stat you want to maximize (enter 0 for physical, 12 for poise, etc).
- If you want to ignore specific pieces (if, for example, you can't access those pieces yet), enter them when prompted for "ignore keywords".
- When prompted, enter the weight you calculated earlier.
- The program will calculate all possible armor sets within your weight limitation, and produce the set with the highest chosen stat within your weight limitation.

# Additional Info
- Some armor stats on the Fextralife Wiki pages for all components of a particular equipment slot sometimes differ from the actual game data, or the data on a particular component's dedicated page.
- This program is only as accurate as the Fextralife Wiki.
- Poise only has significant increments in numbers divisible by 41, 45, 51, 53, 58, 69, 75, 101. This program does not care about that.
