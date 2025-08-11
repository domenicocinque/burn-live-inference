# Burn live inference

A simple HTTP API for MNIST digit classification built with Rust, using the Burn ML framework and Axum web server.

The model is loaded from an ONNX file. Part of the code is derived from [this tutorial](https://github.com/tracel-ai/burn/tree/main/examples/onnx-inference).

## API Endpoints

### Health Check
```http
GET /status
```

**Response:**
```
200 OK
"Ok"
```

### Predict Digit
```http
POST /predict
Content-Type: application/json

{
    "image_b64": "<base64-encoded-image>"
}
```

## Usage Examples

### Using curl
```bash
# Health check
curl http://localhost:8000/status

# Predict digit
curl --location 'http://127.0.0.1:8000/predict' \
--header 'Content-Type: application/json' \
--data '{
    "image_b64": "iVBORw0KGgoAAAANSUhEUgAAABwAAAAcCAAAAABXZoBIAAAAwElEQVR4nGNgGDaAEUKFpD77sfTFHeyS9xQYGBg+X4UKPuk6A6ZZINxU/Wtahg4Wj2UZGP68lmR4dAZZJxgIGp4xZWD4ceu6UM40Bhwg+O9FIVxyYi//B0OZTBiS2aLvb+LSaP3znx0uOYbWf7tZcclxnv1hhVNj3b9tOOW8f3+wxCUnfPffMlxyzKf/3VbGJan2758vLjn5B/+KkcMaBbT++2eCS872E7okUtja8DDc/YIiCY1sCLjo/A6XsZgAAGkRPJA1cOMmAAAAAElFTkSuQmCC"
}'
```
