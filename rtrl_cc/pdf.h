#ifndef PDF_H
#define PDF_H

#include "onb.h"
#include "vec3.h"

class pdf {
   public:
    virtual ~pdf() {}

    virtual double value(const vec3& direction) const = 0;
    virtual vec3 generate() const = 0;
};

class sphere_pdf : public pdf {
   public:
    sphere_pdf() {}

    double value(const vec3& direction) const override { return 1 / (4 * pi); }

    vec3 generate() const override { return random_unit_vector(); }
};

#endif
