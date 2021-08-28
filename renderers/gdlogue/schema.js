export const schema = {
	"type": "array",
	"items": {
		"type": "object",
		"properties": {
			"id": { "type": ["string", "null"] },
			"type": { "type": ["string", "null"] },
			"goto": { "type": ["string", "null"] },
			"speaker": {"type": "string"},
			"text": {"type": "string"},
			"diversion": {
				"type": "array",
				"items": {
					"type":"object",
					"properties": {
						"goto": {"type": "string"},
						"target": {"type": "string"},
						"qual": {"type": "string"},
						"text": {"type": "string"}
					},
					"required": ["goto"]
				}
			}
		},
		"required": ["id", "type", "goto"]
	}
}
