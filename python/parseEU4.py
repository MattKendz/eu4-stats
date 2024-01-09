import argparse
import ClauseWizard
from datetime import date
from datetime import datetime
import json
import os
import re
import shutil
import sys
from zipfile import ZipFile

sys.setrecursionlimit(2000)

# os.rename("orig.eu4", "orig.zip")
parser = argparse.ArgumentParser()

parser.add_argument("-f", "--file", help="EU4 file")

args = parser.parse_args()

if not os.path.exists(args.file):
	print(args.file, 'does not exist.')
	exit(1)

if not args.file.endswith('.eu4'):
	print(args.file, 'does not end with \'.eu4\'')
	exit(1)

folder_name = os.path.basename(args.file)[:-3]
zipped_file = folder_name + 'zip'
file_path = os.path.join(folder_name, zipped_file)

# Uncomment to run on new file
if not os.path.isdir(folder_name):
	os.mkdir(folder_name)
	shutil.copyfile(args.file, file_path)
	with ZipFile(file_path, 'r') as zObject:
		zObject.extractall(path=folder_name)

localisation_map = {}

with open('anb_countries_l_english.yml', 'r', encoding = 'utf-8') as yml:
	lines = yml.readlines()
	for line in lines:
		stripped = line.strip()
		if re.fullmatch(r'\w\d{2}:0 \".*\"', stripped):
			localisation_map[stripped[:3]] = stripped[stripped.index('"') + 1:-1]

buildings_values = {
	'courthouse': 100,
	'town_hall': 200,
	'university': 300,
	'workshop': 100,
	'counting_house': 400,
	'temple': 100,
	'cathedral': 300,
	'shipyard': 100,
	'grand_shipyard': 300,
	'dock': 100,
	'drydock': 300,
	'marketplace': 100,
	'trade_depot': 300,
	'stock_exchange': 400,
	'coastal_defense': 100,
	'naval_battery': 200,
	'barracks': 100,
	'training_fields': 300,
	'regimental_camp': 200,
	'conscription_center': 400,
	'fort_15th': 200,
	'fort_16th': 400,
	'fort_17th': 600,
	'fort_18th': 800,
	'farm_estate': 500,
	'ramparts': 500,
	'impressment_offices': 500,
	'wharf': 500,
	'textile': 500,
	'weapons': 500,
	'state_house': 500,
	'plantations': 500,
	'tradecompany': 500,
	'soldier_households': 500,
	'mills': 500,
	'furnace': 500,
	'mage_tower': 500,
	'fort_magic': 500,
	'native_earthwork': 100,
	'native_fortified_house': 200,
	'native_storehouse': 100,
	'native_longhouse': 100,
	'native_great_trail': 100,
	'native_three_sisters_field': 100,
}

print('Extracting meta...')
current_date = None

with open(os.path.join(folder_name, 'meta'), 'r', encoding='iso-8859-1') as f:
		res = ClauseWizard.cwparse(f.read())
		res_dict = ClauseWizard.cwformat(res)
		current_date = datetime.strptime(res_dict.get('date'), '%Y-%m-%d').date()

print('Finished extracting meta.')
print(current_date)
print('Extracting gamestate...')

if not os.path.exists(os.path.join(folder_name, 'parsed_gamestate')):
	with open(os.path.join(folder_name, 'gamestate'), 'r', encoding='iso-8859-1') as f:
		res = ClauseWizard.cwparse(f.read())
		res_dict = ClauseWizard.cwformat(res)
		with open(os.path.join(folder_name, 'parsed_gamestate'), 'w', encoding='utf-8') as outf:
			outf.write(json.dumps(res_dict, indent=2, ensure_ascii=False))

print('Finished extracting gamestate.')
print('Parsing gamestate...')

game_start = date(1444, 11, 11)
output_data = []

with open(os.path.join(folder_name, 'parsed_gamestate'), 'r', encoding='utf-8') as json_file:
	game_data = json.load(json_file)
	pc = game_data.get('players_countries')
	players = {pc[i]: pc[i + 1] for i in range(0, len(pc), 2)}
	print(players)

	country_list = game_data.get('countries')
	province_list = game_data.get('provinces')

	for country in country_list:
	# for player_name, country in players.items():
		country_data = {}
		country_json = game_data.get('countries').get(country)

		country_data['country_name'] = country
		if country in localisation_map:
			country_data['country_name'] = localisation_map[country]

		if not country_json.get('raw_development'):
			continue

		

		country_data['tag'] = country
		country_data['total_dev'] = int(country_json.get('raw_development'))
		country_data['real_dev'] = round(country_json.get('development'), 2)
		country_data['gp_score'] = int(country_json.get('great_power_score') or 0)

		monarch_history = {}

		powers = [
			country_json.get('powers')[0] + sum((country_json.get('adm_spent_indexed') or {0: 0}).values()),
			country_json.get('powers')[1] + sum((country_json.get('dip_spent_indexed') or {0: 0}).values()),
			country_json.get('powers')[2] + sum((country_json.get('mil_spent_indexed') or {0: 0}).values()),
		]

		country_data['total_mana'] = str(sum(powers)) + '<br>' + '/'.join(str(v) for v in powers)

		country_data['tech'] = '/'.join(str(v) for v in country_json.get('technology').values())
		country_data['total_ideas'] = sum(country_json.get('active_idea_groups').values())
		country_data['curr_manpower'] = int(country_json.get('manpower') * 1000)
		country_data['max_manpower'] = int(country_json.get('max_manpower') * 1000)

		history = country_json.get('history')
		monarchs = []
		for d, info in history.items():
			if not re.fullmatch(r'\d{4}\.\d+\.\d+', d):
				continue
			formatted_date = datetime.strptime(d, '%Y.%m.%d').date()
			if isinstance(info, list):
				for i in info:
					if "monarch" in i:
						monarchs.append([
							i.get("monarch").get("ADM"),
							i.get("monarch").get("DIP"),
							i.get("monarch").get("MIL"),
							formatted_date,
						])
					elif "monarch_heir" in i:
						monarchs.append([
							i.get("monarch_heir").get("ADM"),
							i.get("monarch_heir").get("DIP"),
							i.get("monarch_heir").get("MIL"),
							formatted_date,
						])
			else:
				if "monarch" in info:
					if isinstance(info.get("monarch"), list):
						monarchs.append([
							info.get("monarch")[0].get("ADM"),
							info.get("monarch")[0].get("DIP"),
							info.get("monarch")[0].get("MIL"),
							formatted_date,
						])
					else:
						monarchs.append([
							info.get("monarch").get("ADM"),
							info.get("monarch").get("DIP"),
							info.get("monarch").get("MIL"),
							formatted_date,
						])
				elif "monarch_heir" in info:
					if isinstance(info.get("monarch_heir"), list):
						monarchs.append([
							info.get("monarch_heir")[0].get("ADM"),
							info.get("monarch_heir")[0].get("DIP"),
							info.get("monarch_heir")[0].get("MIL"),
							formatted_date,
						])
					else:
						monarchs.append([
							info.get("monarch_heir").get("ADM"),
							info.get("monarch_heir").get("DIP"),
							info.get("monarch_heir").get("MIL"),
							formatted_date,
						])

		# In case EU4 stores multiple leaders before game start, remove them
		first_king_index = 0
		for i in range(0, len(monarchs)):
			king = monarchs[i]
			if king[-1] > game_start:
				continue
			if i < len(monarchs) - 1 and monarchs[i+1][-1] <= game_start:
				first_king_index = i
		monarchs = monarchs[first_king_index:]
		monarchs[0][-1] = game_start

		avg_monarch = [0.0, 0.0, 0.0]
		for i in range(1, len(monarchs)):
			days = (monarchs[i][-1] - monarchs[i-1][-1]).days
			avg_monarch[0] += (days * float(monarchs[i-1][0]))
			avg_monarch[1] += (days * float(monarchs[i-1][1]))
			avg_monarch[2] += (days * float(monarchs[i-1][2]))
		days = (current_date - monarchs[-1][-1]).days
		avg_monarch[0] += (days * float(monarchs[-1][0]))
		avg_monarch[1] += (days * float(monarchs[-1][1]))
		avg_monarch[2] += (days * float(monarchs[-1][2]))
		avg_monarch_stats = [round(i / float((current_date - game_start).days), 3) for i in avg_monarch]

		country_data['avg_monarch'] = str('%.3f' % sum(avg_monarch_stats)) + '<br>' + '/'.join(str(v) for v in avg_monarch_stats)
		country_data['income'] = round(country_json.get('estimated_monthly_income'), 2)
		country_data['provinces'] = len(country_json.get('owned_provinces'))
		
		num_buildings = 0
		buildings_value = 0

		for province in country_json.get('owned_provinces'):
			province_data = province_list.get('-%d' % province)
			if not province_data.get('buildings'):
				continue
			num_buildings += len(province_data.get('buildings'))
			for buildings in province_data.get('buildings').keys():
				if buildings not in buildings_values:
					print(buildings + ' not found')
				buildings_value += buildings_values[buildings]

		country_data['num_buildings'] = num_buildings
		country_data['buildings_value'] = buildings_value
		country_data['buildings_per_province'] = round(float(num_buildings) / country_data['provinces'], 3)
		#country_data['avg_buildings_value'] = round(buildings_value / max(num_buildings, 1), 3)

		country_data['inno'] = country_json.get('innovativeness') or 0.0
		country_data['absolutism'] = country_json.get('absolutism') or 0.0
		country_data['avg_dev'] = round(country_data['total_dev'] / country_data['provinces'], 3)
		country_data['avg_dev_real'] = round(country_data['real_dev'] / country_data['provinces'], 3)
		country_data['player'] = ''
		if country_json.get('human'):
			country_data['player'] = list(players.keys())[list(players.values()).index(country)]

		country_data['army_professionalism'] = round(100.0 * (country_json.get('army_professionalism') or 0.0), 2)
		

		output_data.append(country_data)
		# print(country_data)
		# print()

json_object = json.dumps(output_data, indent=4)
with open(os.path.join(folder_name, "parsed_data.json"), "w") as outfile:
	outfile.write(json_object)