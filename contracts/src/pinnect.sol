// SPDX-License-Identifier: MIT
pragma solidity >=0.7.0 <0.9.0;

contract Pinnect {
    struct Map {
        string name;
        string description;
        string image;
    }

    struct Location {
        address owner;
        address map_id;
        string name;
        int x;  // in fixed-point format
        int y;  // in fixed-point format
        string tags;
        string image;
        bool is_on_chain;
    }

    Map[] public maps;
    Location[] public locations;

    event MessageToLayer2(string message);
    event MessageReceived(string message);

    function createMap(string memory name, string memory description, string memory image) public {
        Map memory newMap = Map({
            name: name,
            description: description,
            image: image
        });
        maps.push(newMap);
        emit MessageToLayer2("MAP_CREATED");
    }

    function createLocation(address owner, address map_id, string memory name, int x, int y, string memory tags, string memory image) public {
        Location memory newLocation = Location({
            owner: owner,
            map_id: map_id,
            name: name,
            x: x,
            y: y,
            tags: tags,
            image: image,
            is_on_chain: false
        });
        locations.push(newLocation);
        emit MessageToLayer2("LOCATION_CREATED");
    }

    function receiveMessageFromLayer2(string memory message) public {
        if(keccak256(abi.encodePacked(message)) == keccak256(abi.encodePacked("MAP_CREATED"))) {
            emit MessageReceived("A map was created in Layer 2");
        } else if(keccak256(abi.encodePacked(message)) == keccak256(abi.encodePacked("LOCATION_CREATED"))) {
            emit MessageReceived("A location was created in Layer 2");
        } else {
            emit MessageReceived("Unknown message received from Layer 2");
        }
    }
}