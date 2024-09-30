"""
Developed by: 	Joctan Porras Esquivel
Date:			09-27-2024
Description:	This script is used to manage JSON files in a custom http server.
                It only works with JSON files and only have 5 options:
                    0: GET
                    1: POST
                    2: PUT
                    3: DELETE
                    4: PATCH
                The script will print a JSON text to be used in the server.
                If an error occurs, the script will print a JSON error message.
                The options are already defined in the server.
                Those are:
                    GET: Returns all the data in the JSON file, it is Formula 1 information.
                    POST: Adds a new team with all the data to the JSON file.
                    PUT: Updates the data of a team in the JSON file.
                    DELETE: Deletes a team from the JSON file.
                    PATCH: Updates a specific data of a team in the JSON file.
                The script also receives a name of a JSON file to be used in the POST, PUT, DELETE and PATCH options.
                It will search the file in the /tmp directory of this project.
"""
import argparse
import json
import sys
from pathlib import Path as pl



def load_json_file(file_path):
    try:
        with open(file_path, 'r') as f:
            return json.load(f)
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

def print_json(data):
    print(json.dumps(data, indent=4))

def write_json_file(file_path, data):
    try:
        with open(file_path, 'w') as f:
            json.dump(data, f, indent=4)
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)





def main():
    # Parse arguments
    parser = argparse.ArgumentParser()
    parser.add_argument('--option', type=int, required=True, choices=[0, 1, 2, 3, 4], help="0: GET, 1: POST, 2: PUT, 3: DELETE, 4: PATCH")
    parser.add_argument('--name', type=str, help="Name of body JSON file\nUsually located in /tmp")
    args = parser.parse_args()

    if args.option in [1, 2, 3, 4] and args.name is None:
        print_json({"error": "Path is required for POST, PUT, DELETE and PATCH options"})
        sys.exit(1)

    # Getting arguments into variables
    option = args.option
    name = args.name

    

    

    # CONSTANTS
    CURRENT_FILE_PATH = pl(__file__).resolve().parent
    F1_DATA_PATH = pl(CURRENT_FILE_PATH.parent) / "data" 
    F1_DATA_PATH = F1_DATA_PATH / "f1_data.json"

    # Control data
    response = {}
    request = {}
    if name != "" and option != 0:
        name = pl(CURRENT_FILE_PATH.parent/ 'tmp' / name)
        if not name.is_file():
            print_json({"error": "File not found"})
            sys.exit(1)

    f1_data = load_json_file(F1_DATA_PATH)


    # Selecting the option
    if option == 0: # GET
        response = f1_data
        print_json(response)
        sys.exit(0)
    elif option == 1: # POST
        request = load_json_file(name)
        new_team = {
            "name": request["body"]["name"],
            "drivers": request["body"]["drivers"]
        }
        f1_data["teams"].append(new_team)
        write_json_file(F1_DATA_PATH, f1_data)
        print_json({"message": "Team added successfully"})
        sys.exit(0)
    elif option == 2: # PUT
        request = load_json_file(name)
        for i, team in enumerate(f1_data["teams"]):
            if team["name"] == request["team"]:
                new_data = {
                    "name": request["body"]["name"],
                    "drivers": request["body"]["drivers"]
                }
                f1_data["teams"][i] = new_data
                write_json_file(F1_DATA_PATH, f1_data)
                print_json({"message": "Team updated successfully"})
                sys.exit(0)
        # TODO: Print error message in right format, Im not sure if print a message code and return the json in the server side or send the json from here
        #       It will be a tomorrows problem
        #print_json({"error": "Team not found"})
        print(404)
        sys.exit(1)
    elif option == 3: # DELETE
        request = load_json_file(name)
        for i, team in enumerate(f1_data["teams"]):
            if team["name"] == request["team"]:
                del f1_data["teams"][i]
                write_json_file(F1_DATA_PATH, f1_data)
                print_json({"message": "Team deleted successfully"})
                sys.exit(0)
        print(404)
        sys.exit(1)
    elif option == 4: # PATCH
        request = load_json_file(name)
        for i, team in enumerate(f1_data["teams"]):
            if team["name"] == request["team"]:
                for driver in team["drivers"]:
                    if driver["name"] == request["driver"]:
                        
                        for key, value in request["body"].items():
                            driver[key] = value
                        write_json_file(F1_DATA_PATH, f1_data)
                        print_json({"message": "Driver updated successfully"})
                        sys.exit(0)
                        
                print(404)
        print(404)
        sys.exit(1)

        




if __name__ == "__main__":
    main()
