#version 400

in vec2 screen_position;
out vec4 color;

uniform float aspect_ratio;
uniform float x;
uniform float y;
uniform float zoom;
uniform int iterations;
uniform int cycle_iters;
uniform sampler1D gradient;


void main() {
	
	vec2 c = vec2(screen_position.x - 0.5, (screen_position.y - 0.5) * aspect_ratio) * zoom + vec2(x, y);
	vec2 z = c;
	int count = 0;
	float t;
	while (0 == 0) {
		z = vec2(z.x * z.x - z.y * z.y, 2 * z.x * z.y) + c;
		count += 1;
		
		if (dot(z, z) >= 4) {
			t = float(count) / cycle_iters;
			color = texture(gradient, t);
			break;
		} else if (count >= iterations) {
			color = vec4(0, 0, 0, 0);
			break;
		}
	}
	
}

