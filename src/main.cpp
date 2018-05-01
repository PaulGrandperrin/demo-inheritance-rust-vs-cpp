#include <iostream>
#include <cmath>

using namespace std;

class FlatObject {
public:
    virtual float get_surface() const = 0;
    virtual float get_thickness() const = 0;
    
    virtual float get_volume() const {
        cout << "  computing volume from FlatObject" << endl;
        return get_surface() * get_thickness();
    }
};

class ThickObject: public FlatObject {
protected:
    float thickness;

public:
    ThickObject(float thickness): thickness(thickness) {
        cout <<"  constructing ThickObject" << endl;
    }

    virtual float get_thickness() const override final {
        cout <<"  accessing thickness" << endl;
        return this->thickness;
    }
};

class ThickCircle: public ThickObject {
    float radius;

public:
    ThickCircle(float thickness, float radius)
    : ThickObject(thickness), radius(radius) {
        cout <<"  constructing ThickCircle" << endl;
    }

    virtual float get_surface() const override {
        cout << "  computing surface from ThickCircle" << endl;
        return M_PI * this->radius * this->radius;
    }
};

class ThickRectangle final: public ThickObject {
    float height;
    float width;

public:
    ThickRectangle(float thickness, float height, float width)
    : ThickObject(thickness), height(height), width(width) {
        cout <<"  constructing ThickRectangle" << endl;
    }

    virtual float get_surface() const override {
        cout << "  computing surface from ThickRectangle" << endl;
        return this->width * this->height;
    }

    virtual float get_volume() const override { // overriding default method
        cout << "  computing volume from ThickRectangle" << endl;
        return this->width * this->height * this->thickness;
    }
};

// generic function using dynamic dispatch through vtable indirection

void print_volume(const FlatObject& o) {
    auto volume = o.get_volume();
    cout << "volume: " << volume << endl;
}

int main()
{
    cout << "ThickCircle of thickness 2 and radius 10" << endl;
    const auto c = ThickCircle(2,10);
    print_volume(c);
    cout << endl;
    cout << "ThickRectangle of thickness 3 and dimentions 2*4" << endl;
    const auto r = ThickRectangle(3, 2, 4);
    print_volume(r);
}