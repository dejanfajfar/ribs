```
 ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄ 
█       █       █   █
█   ▄   █    ▄  █   █
█  █▄█  █   █▄█ █   █
█       █    ▄▄▄█   █
█   ▄   █   █   █   █
█▄▄█ █▄▄█▄▄▄█   █▄▄▄█

```

# Contract

## Battlefield

```json
{
    "height": 16,
    "width": 16,
    "combatants": [
        {
            "name": "test1",
            "dmg": 5,
            "hp": 100
        }
    ]
}
```

## [POST] /battle/

```json
{
	"map": {
		"height": 16,
		"width": 16
	},
	"combatants": [
		{
            "name": "test1",
            "dmg": 2,
            "hp": 20
		},
		{
            "name": "test2",
            "dmg": 2,
            "hp": 20
		},
		{
            "name": "test3",
            "dmg": 2,
            "hp": 20
		}
	]
}
```

## [POST] /combatants

```json
{
	"name": "test3",
	"dmg": 5,
	"hp": 400,
	"avatar": "image.jpg"
}
```