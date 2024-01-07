#version 400

in vec2 pos;
out vec4 color;

uniform float aspect_ratio;
uniform double x;
uniform double y;
uniform double zoom;
uniform int iterations;
uniform int cycle_iters;
uniform sampler1D gradient;


void main() {
	
	dvec2 c = dvec2(pos.x - 0.5, (pos.y - 0.5) * aspect_ratio) * zoom + dvec2(x, y);
	dvec2 z = c;
	int count = 0;
	float t;
	while (0 == 0) {
		z = dvec2(z.x * z.x - z.y * z.y, 2 * z.x * z.y) + c;
		count += 1;
		
		if (dot(z, z) >= 4) {
			t = clamp(float(count % cycle_iters) / cycle_iters, 0, 1);
			color = texture(gradient, t);
			break;
		} else if (count >= iterations) {
			color = vec4(0, 0, 0, 0);
			break;
		}
	}
	
}

