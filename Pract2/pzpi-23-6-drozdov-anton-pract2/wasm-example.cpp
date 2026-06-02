#include <emscripten.h>
#include <vector>

struct Rectangle {
    float x, y, width, height;
};

// Зберігаємо вектор об'єктів у пам'яті WASM
std::vector<Rectangle> canvasObjects;

// Функція експортується в JavaScript і викликається 60 разів на секунду (RequestAnimationFrame)
EMSCRIPTEN_KEEPALIVE
void renderFrame() {
    for (const auto& rect : canvasObjects) {
        // Прямий виклик WebGL контексту для малювання бінарних буферів
        drawRectangleViaWebGL(rect.x, rect.y, rect.width, rect.height);
    }
}

EMSCRIPTEN_KEEPALIVE
void updateObject(int index, float newX, float newY) {
    if (index < canvasObjects.size()) {
        canvasObjects[index].x = newX;
        canvasObjects[index].y = newY;
    }
}
