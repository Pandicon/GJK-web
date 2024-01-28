# GET /supl

required permissions: `READ_SUBSTITUTIONS`

replies with json data with substitutions:

```json
{
	"curr_day":<uint current_day_id>,
	"days":[
		{
			"id":<uint day_id>,
			"hours":[
				{
					"missing_classes":[
						<delta>,
						// ...>
					],
					"missing_reachers":[
						<delta>,
						// ...>
					],
					"missing_rooms":[
						<delta>,
						// ...
					],
					"class_changes":{
						<string class>:[
							<delta>,
							// ...
						],
						// ...
					},
					"teacher_changes":{
						<string teacher>:[
							<delta>,
							// ...
						],
						// ...
					}
				}
				// ...
			]
		},
		// ...
	]
}
```

where `<delta>` is

```json
{
	"value":<string value>,
	"added":<bool value_added>,
	"time":<string time_added>,
	"ut":<uint unix_timestamp>
}
```

## Errors

### 401 Unauthorized

If the user doesn't have required permissions, the server replies with status code `401 Unauthorized` and no body.

### 404 Not found

If the server has `supl_fetch_enabled` option in config set to `false`, then this endpoint doesn't exist
