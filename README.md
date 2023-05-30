# elden_ring_build_optimizer
Find optimal armor set for any stat within weight limitations.

# requirements
- python3
- `pip3 install -r requirements.txt`

# Usage
- In game, equip your weapon loadout and amulet loadout, and remove all armor.
- Take your carry weight max capacity, and multiply it by .299 if you want a light roll, .699 if you want a medium roll, or .999 if you want a heavy roll.
- Subtract your current equip load from the result of the step above. The program will later prompt for this number.
- `python3 main.py`
- Follow the prompts to choose the stat you want to maximize (this will typically be `physical` or `poise`).
- You will see a list of the armor pieces with the best ratios. Ratios are calculated via `your_chosen_stat / weight`.
- When prompted, enter the weight you calculated earlier.
- You should now see a list of armor sets. The best sets for your chosen stat will be at the bottom.

# Additional Info
- The armor sets are calculated using the 10 components from each armor slot that have the best ratios, as well as the 10 from each slot that have the highest raw value for your chosen stat.
- It is technically possible to calculate sets using ALL components, but most people won't have enough ram (or time) for this, and the results would likely not differ by much anyway.
- If this script ever stops working, it's most likely due to the wiki reformatting their tables or moving pages. Open an issue if this happens and I'll try to get it updated.
