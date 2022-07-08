#!/usr/bin/env python3
import requests
from bs4 import BeautifulSoup
from itertools import product
import time

def remove_duplicates(a, b):
    results = []
    for i in a + b:
        if i["name"] not in [i["name"] for i in results]:
            results.append(i)
    return results


def empty_piece(slot):
    piece = {
        "slot": slot,
        "name": "empty",
        "physical": 0.0,
        "slash": 0.0,
        "strike": 0.0,
        "pierce": 0.0,
        "magic": 0.0,
        "fire": 0.0,
        "lightning": 0.0,
        "holy": 0.0,
        "immunity": 0.0,
        "robustness": 0.0,
        "focus": 0.0,
        "vitality": 0.0,
        "poise": 0.0,
        "weight": 0.0,
        "ratio": 0.0,
    }
    return piece

def collect_components():
    components = {
            "helm": "https://eldenring.wiki.fextralife.com/Helms",
            "chest": "https://eldenring.wiki.fextralife.com/Chest+Armor",
            "gauntlets": "https://eldenring.wiki.fextralife.com/Gauntlets",
            "legs": "https://eldenring.wiki.fextralife.com/Leg+Armor",
        }

    armor_pieces = []

    for slot, component in components.items():
        r = requests.get(component)
        soup = BeautifulSoup(r.text, 'html.parser')

        table = soup.find("table", attrs = {"class": "wiki_table sortable searchable"})
        tbody = table.find("tbody")
        rows = tbody.find_all("tr")

        for row in rows:
            children = [i for i in row.children]
            
            # for index, data in enumerate(children):
            #    print(f"{index}: {data.text}")

            piece = {
                "slot": slot,
                "name": children[1].text,
                "weight": children[29].text,
                "physical": children[3].text,
                "strike": children[5].text,
                "slash": children[7].text,
                "pierce": children[9].text,
                "magic": children[11].text,
                "fire": children[13].text,
                "lightning": children[15].text,
                "holy": children[17].text,
                "immunity": children[19].text,
                "robustness": children[21].text,
                "focus": children[23].text,
                "vitality": children[25].text,
                "poise": children[27].text,
                "ratio": 0.0,
            }

            armor_pieces.append(piece)

    return armor_pieces

def print_available_stats():
    stats = [
            "physical",
            "strike",
            "slash",
            "pierce",
            "magic",
            "fire",
            "lightning",
            "holy",
            "immunity",
            "robustness",
            "focus",
            "vitality",
            "poise",
        ]
    for i in stats:
        print(i)

    return stats

if __name__ == "__main__":
    print("collecting data from the wiki...")
    pre_armor_pieces = collect_components()

    keys = print_available_stats()

    maximize_stat = ""
    while maximize_stat not in keys:
        maximize_stat = input("enter the stat you want to maximize: ")
        if maximize_stat not in keys:
            print(f"please select from the following stats: ")
            print_available_stats()

    armor_pieces = []
    for armor in pre_armor_pieces:
        try:
            armor["ratio"] = float(armor[maximize_stat]) / float(armor['weight'])
            # ensure we are only using pieces that won't error out later.
            for key in armor.keys():
                if key not in ["name", "slot"]:
                    armor[key] = float(armor[key])

            armor_pieces.append(armor)
        except Exception as e:
            print(f"skipping {armor['name']} due to error: {e}")

    print("finding the 10 most weight efficient components and the 10 highest value components...")
    time.sleep(2)

    # show the 20 best components
    sorted_armor = sorted(armor_pieces, key=lambda x: x["ratio"])
    sorted_armor_by_max = sorted(armor_pieces, key=lambda x: x[maximize_stat])
    
    for armor_piece in sorted_armor[-10:] + sorted_armor_by_max[-10:]:
        print()
        for key, value in armor_piece.items():
            if key in ["slot", "name"]:
                print(f"{key}: {value}")
            else:
                print(f"{key}: {format(value, '.2f')}")

    # collect components by slot
    chest = [i for i in sorted_armor if i["slot"] == "chest"]
    legs = [i for i in sorted_armor if i["slot"] == "legs"]
    gauntlets = [i for i in sorted_armor if i["slot"] == "gauntlets"]
    helm = [i for i in sorted_armor if i["slot"] == "helm"]

    # gather highest ratio and highest choice stat components by slot
    chest_ratio = sorted(chest, key=lambda x: x["ratio"])
    chest_max = sorted(chest, key=lambda x: x[maximize_stat])
    legs_ratio = sorted(legs, key=lambda x: x["ratio"])
    legs_max = sorted(legs, key=lambda x: x[maximize_stat])
    gauntlets_ratio = sorted(gauntlets, key=lambda x: x["ratio"])
    gauntlets_max = sorted(gauntlets, key=lambda x: x[maximize_stat])
    helm_ratio = sorted(helm, key=lambda x: x["ratio"])
    helm_max = sorted(helm, key=lambda x: x[maximize_stat])

    print()
    max_weight = float(input("enter your weight limitation for armor: "))

    # we need an empty piece for each slot, create them here.
    chest_empty = empty_piece("chest")
    legs_empty = empty_piece("legs")
    gauntlets_empty = empty_piece("gauntlets")
    helm_empty = empty_piece("helm")

    # remove duplicate pieces
    best_chest = remove_duplicates(chest_ratio[-10:], chest_max[-10:])
    best_legs = remove_duplicates(legs_ratio[-10:], legs_max[-10:])
    best_gauntlets = remove_duplicates(gauntlets_ratio[-10:], gauntlets_max[-10:])
    best_helms = remove_duplicates(helm_ratio[-10:], helm_max[-10:])





    # get a list of all possible builds using the 20 highest efficiency pieces,
    # and the 20 highest choice stat pieces from each slot.
    all_builds = list(
            product(
                best_chest + [chest_empty],
                best_legs + [legs_empty],
                best_gauntlets + [gauntlets_empty],
                best_helms + [helm_empty]
            )
        )

    # get the specs for each armor set.
    values = []
    for build in all_builds:
        build_specs = {
            "weight": 0.0,
            "physical": 0.0,
            "strike": 0.0,
            "slash": 0.0,
            "pierce": 0.0,
            "magic": 0.0,
            "fire": 0.0,
            "lightning": 0.0,
            "holy": 0.0,
            "immunity": 0.0,
            "robustness": 0.0,
            "focus": 0.0,
            "vitality": 0.0,
            "poise": 0.0,
        }

        for component in build:
            for key, value in component.items():
                if key not in ["name", "slot", "ratio"]:
                    build_specs[key] += value

        values.append(build_specs)

    # remove builds that strictly exceed the weight limitation.
    results = []
    for val, build in zip(values, all_builds):
        if val["weight"] > max_weight:
            continue

        result = {}
        result["chest"] = [i["name"] for i in build if i["slot"] == "chest"][0]
        result["helm"] = [i["name"] for i in build if i["slot"] == "helm"][0]
        result["gauntlets"] = [i["name"] for i in build if i["slot"] == "gauntlets"][0]
        result["legs"] = [i["name"] for i in build if i["slot"] == "legs"][0]
        for key, value in val.items():
            result[key] = value

        results.append(result)

    sorted_results = sorted(results, key=lambda x: x[maximize_stat])

    for i in sorted_results[-25:]:
        print()
        for key, value in i.items():
            if key in ["name", "slot", "chest", "helm", "gauntlets", "legs"]:
	            print(f"{key}: {value}")
            else:
                    print(f"{key}: {format(value, '.2f')}")

    print()
    input("Press any key to quit.")
    exit()
