[![Docker Build and Deploy](https://github.com/rossbrandon/flora-api/actions/workflows/build-deploy.yml/badge.svg?branch=main)](https://github.com/rossbrandon/flora-api/actions/workflows/build-deploy.yml)

# flora-api

Basic REST API to support the Data Flora concept.

For more context about this concept, see my blog post on my portfolio site:
[Designing Data Systems for Visibility & Management](https://rossbrandon.dev/posts/designing-data-systems-for-visibility/)

UI Repo: [flora-dash](https://github.com/rossbrandon/flora-dash)

See it in action at https://dataflora.dev

## Technologies Used

- [Rust](https://www.rust-lang.org/) lang
- [Tokio](https://tokio.rs/) async Rust runtime
- [Axum](https://github.com/tokio-rs/axum) for the REST API framework
- [MongoDB Atlas](https://www.mongodb.com/atlas/database) free Cloud DB hosting

## Running Locally

### Prerequisites

Install Rust tooling: [rustup](https://www.rust-lang.org/tools/install)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Running via Cargo

Install Cargo crates
```bash
cargo install --path .
```

Run the server!
```bash
cargo run
```

If successful, a message will appear similar to:

```bash
2023-11-26T16:49:46.788799Z DEBUG flora_api::http: http server listening on 0.0.0.0:8080
```

The API can be accessed at `localhost:8080`.

### Running via Docker

*Prerequisite:* Install [Docker Desktop](https://www.docker.com/products/docker-desktop/)

Build image:
```bash
docker build -t rosstafarian/flora-api .
```

Run image:
```bash
docker run -d -p 8080:8080 --rm --name flora-api rosstafarian/flora-api
```

## Deployment

This service is [Dockerized](https://github.com/rossbrandon/flora-api/blob/main/Dockerfile) and hosted openly
on Docker Hub: [rosstafarian/flora-api](https://hub.docker.com/r/rosstafarian/flora-api).

The API is currently deployed via [Github Actions](https://github.com/features/actions)
to [AWS Lightsail Containers](https://aws.amazon.com/blogs/aws/lightsail-containers-an-easy-way-to-run-your-containers-in-the-cloud/)

See [build-deploy.yml](https://github.com/rossbrandon/flora-api/blob/main/.github/workflows/build-deploy.yml)
for build and deploy script

### Kubernetes

This service is enabled to use [Helm Charts](https://github.com/rossbrandon/flora-api/tree/main/helm-chart)
for deployment on a Kubernetes cluster, if desired.


## API Guide

### Ping

Request: `GET http://localhost:8080/ping`

Response:

```bash
pong
```

### Get Clients

Gets all clients from the `clients` DB collection.

Request: `GET http://localhost:8080/api/v1/clients`

Response:

```bash
[
    {
        "clientId": "internal",
        "name": "Internal Pipelines",
        "clientType": "Internal",
        "deviceType": "System",
        "description": "Internal system-to-system data pipelines.",
        "userAgent": "grpc-rust/9.2.1"
    },
    {
        "clientId": "3e7780b8-7b57-44d2-ab6c-163ee1e18bb3",
        "name": "User iOS Device",
        "clientType": "External",
        "deviceType": "Mobile",
        "description": "End user device",
        "userAgent": "Mozilla/5.0 (X11; Linux i686 AppleWebKit/533.1.0 (KHTML, like Gecko) Chrome/36.0.811.0 Safari/533.1.0"
    },
    ...
]
```

### Get Client By ID

Gets client by `clientId` from the `clients` DB collection.

Request: `GET http://localhost:8080/api/v1/clients/{clientId}`

Response:

```bash
{
	"clientId": "internal",
	"name": "Internal Pipelines",
	"clientType": "Internal",
	"description": "Internal system-to-system data pipelines.",
	"deviceType": "System",
	"userAgent": "grpc-rust/9.2.1"
}
```

### Get Flows by Client ID

Gets all data flows for a given `clientId` from the `flows` DB collection.

Request: `GET http://localhost:8080/api/v1/clients/{clientId}/flows`

Response:

```bash
[
    {
		"clientId": "internal",
		"upstream": {
			"upstreamId": "contentIngest",
			"name": "Content Ingestion",
			"total": 1750000,
			"inError": 0,
			"health": "Healthy"
		},
		"downstreams": [
			{
				"downstreamId": "imageAnalyzer",
				"name": "Image Analyzer",
				"expected": 3000,
				"received": 3000,
				"missing": 0,
				"inError": 0,
				"lastReceived": {
					"documentId": "9eb5b72d-a491-42f2-9785-aaed4ca22cb9",
					"timestamp": "2023-11-11T00:42:24.300Z"
				},
				"health": "Healthy"
			},
			{
				"downstreamId": "videoAnalyzer",
				"name": "Video Analyzer",
				"expected": 17000,
				"received": 17000,
				"missing": 0,
				"inError": 0,
				"lastReceived": {
					"documentId": "1b8823a7-0eb9-43cb-a88e-4ca393253b74",
					"timestamp": "2023-11-10T10:49:33.084Z"
				},
				"health": "Healthy"
			},
			{
				"downstreamId": "fileStorage",
				"name": "File Storage",
				"expected": 1750000,
				"received": 751794,
				"missing": 998206,
				"inError": 0,
				"lastReceived": {
					"documentId": "64880e44-fde6-4de1-b5da-7e699e407461",
					"timestamp": "2023-11-12T18:41:16.584Z"
				},
				"health": "Healthy"
			},
			{
				"downstreamId": "metadataStorage",
				"name": "Metadata Storage",
				"expected": 1500,
				"received": 1035,
				"missing": 115,
				"inError": 350,
				"lastReceived": {
					"documentId": "d98c65d0-95ae-41ef-a603-9097afaaf627",
					"timestamp": "2023-11-11T22:30:46.570Z"
				},
				"health": "InvestigationNeeded"
			}
		],
		"health": "InvestigationNeeded",
		"meta": {
			"description": null,
			"systemDiagramUrl": "https://excalidraw.com",
			"runbookUrl": "https://exhausted-flock.biz/runbook",
			"contactSlackChannel": "https://identical-preoccupation.net//slack#contentIngestSupport,"
		}
	},
    ...
]
```

### Get Flow by Client ID & Upstream ID

Gets all data flow for a given `clientId` and `upstreamId` from the `flows` DB collection.

Request: `GET http://localhost:8080/api/v1/clients/{clientId}/flows/{upstreamId}`

Response:

```bash
{
	"clientId": "internal",
	"upstream": {
		"upstreamId": "contentIngest",
		"name": "Content Ingestion",
		"total": 1750000,
		"inError": 0,
		"health": "Healthy"
	},
	"downstreams": [
		{
			"downstreamId": "imageAnalyzer",
			"name": "Image Analyzer",
			"expected": 3000,
			"received": 3000,
			"missing": 0,
			"inError": 0,
			"lastReceived": {
				"documentId": "9eb5b72d-a491-42f2-9785-aaed4ca22cb9",
				"timestamp": "2023-11-11T00:42:24.300Z"
			},
			"health": "Healthy"
		},
		{
			"downstreamId": "videoAnalyzer",
			"name": "Video Analyzer",
			"expected": 17000,
			"received": 17000,
			"missing": 0,
			"inError": 0,
			"lastReceived": {
				"documentId": "1b8823a7-0eb9-43cb-a88e-4ca393253b74",
				"timestamp": "2023-11-10T10:49:33.084Z"
			},
			"health": "Healthy"
		},
		{
			"downstreamId": "fileStorage",
			"name": "File Storage",
			"expected": 1750000,
			"received": 751794,
			"missing": 998206,
			"inError": 0,
			"lastReceived": {
				"documentId": "64880e44-fde6-4de1-b5da-7e699e407461",
				"timestamp": "2023-11-12T18:41:16.584Z"
			},
			"health": "Healthy"
		},
		{
			"downstreamId": "metadataStorage",
			"name": "Metadata Storage",
			"expected": 1500,
			"received": 1035,
			"missing": 115,
			"inError": 350,
			"lastReceived": {
				"documentId": "d98c65d0-95ae-41ef-a603-9097afaaf627",
				"timestamp": "2023-11-11T22:30:46.570Z"
			},
			"health": "InvestigationNeeded"
		}
	],
	"health": "InvestigationNeeded",
	"meta": {
		"description": null,
		"systemDiagramUrl": "https://excalidraw.com",
		"runbookUrl": "https://exhausted-flock.biz/runbook",
		"contactSlackChannel": "https://identical-preoccupation.net//slack#contentIngestSupport,"
	}
}
```

### Get Errors by Upstream ID & Downstream ID

Gets all data flow errors for a given `upstreamId` and `downstreamId` from the `errors` DB collection.

Request: `GET http://localhost:8080/api/v1/errors/upstreams/{upstreamId}/downstreams/{downstreamId}`

Response:

```bash
{
	"clientId": "internal",
	"upstream": {
		"upstreamId": "contentIngest",
		"name": "Content Ingestion",
		"total": 1750000,
		"inError": 0,
		"health": "Healthy"
	},
	"downstreams": [
		{
			"downstreamId": "imageAnalyzer",
			"name": "Image Analyzer",
			"expected": 3000,
			"received": 3000,
			"missing": 0,
			"inError": 0,
			"lastReceived": {
				"documentId": "9eb5b72d-a491-42f2-9785-aaed4ca22cb9",
				"timestamp": "2023-11-11T00:42:24.300Z"
			},
			"health": "Healthy"
		},
		{
			"downstreamId": "videoAnalyzer",
			"name": "Video Analyzer",
			"expected": 17000,
			"received": 17000,
			"missing": 0,
			"inError": 0,
			"lastReceived": {
				"documentId": "1b8823a7-0eb9-43cb-a88e-4ca393253b74",
				"timestamp": "2023-11-10T10:49:33.084Z"
			},
			"health": "Healthy"
		},
		{
			"downstreamId": "fileStorage",
			"name": "File Storage",
			"expected": 1750000,
			"received": 751794,
			"missing": 998206,
			"inError": 0,
			"lastReceived": {
				"documentId": "64880e44-fde6-4de1-b5da-7e699e407461",
				"timestamp": "2023-11-12T18:41:16.584Z"
			},
			"health": "Healthy"
		},
		{
			"downstreamId": "metadataStorage",
			"name": "Metadata Storage",
			"expected": 1500,
			"received": 1035,
			"missing": 115,
			"inError": 350,
			"lastReceived": {
				"documentId": "d98c65d0-95ae-41ef-a603-9097afaaf627",
				"timestamp": "2023-11-11T22:30:46.570Z"
			},
			"health": "InvestigationNeeded"
		}
	],
	"health": "InvestigationNeeded",
	"meta": {
		"description": null,
		"systemDiagramUrl": "https://excalidraw.com",
		"runbookUrl": "https://exhausted-flock.biz/runbook",
		"contactSlackChannel": "https://identical-preoccupation.net//slack#contentIngestSupport,"
	}
}{
	"upstreamId": "contentIngest",
	"downstreamId": "metadataStorage",
	"aggregates": [
		{
			"errorType": "InvalidData",
			"count": 0,
			"message": "Invalid data received",
			"logLink": null
		},
		{
			"errorType": "MissingData",
			"count": 0,
			"message": "Missing data",
			"logLink": null
		},
		{
			"errorType": "NetworkError",
			"count": 350,
			"message": "Network error",
			"logLink": null
		}
	],
	"errors": [
		{
			"upstreamId": "contentIngest",
			"downstreamId": "metadataStorage",
			"documentId": "4ef2baa8-6ce7-41c6-a366-aeab96a98977",
			"errorType": "NetworkError",
			"message": "Could not connect to metadataStorage service: http timeout (10s) reached",
			"logLink": "https://log.dataflora.io/UVsGLJ",
			"timestamp": "2023-11-12T05:26:48.278Z"
		},
		{
			"upstreamId": "contentIngest",
			"downstreamId": "metadataStorage",
			"documentId": "ff6da4a3-6f67-482c-8b98-b4798914ebbd",
			"errorType": "NetworkError",
			"message": "Could not connect to metadataStorage service: http timeout (10s) reached",
			"logLink": "https://log.dataflora.io/T7Pe9n",
			"timestamp": "2023-11-11T23:23:57.542Z"
		},
		...
    ]
}
```