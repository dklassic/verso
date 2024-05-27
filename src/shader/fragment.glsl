#version 300 es

precision mediump float;
in vec2 uv;
vec2 center;
vec2 size;
out vec4 final_color;

float roundedBoxSDF(vec2 CenterPosition, vec2 Size, float Radius) {
    return length(max(abs(CenterPosition)-Size+Radius,0.0))-Radius;
}
void main() {
    vec2 boxPos; // The position of the center of the box (in normalized coordinates)
    vec2 boxBnd; // The half-bounds (radii) of the box (in normalzied coordinates)
    float radius;// Radius

    boxPos = vec2(0, 0);    // center of the screen
    boxBnd = vec2(0.25, 0.25);  // half of the area
    radius = 0.1;

    float alpha = length(max(abs(uv - boxPos) - boxBnd, 0.0)) - radius;
    if(alpha > 0.0)
    {
        final_color = vec4(0, 0, 0, 0);
    }
    else
    {
        final_color = vec4(1, 1, 1, 1);
    }
}
