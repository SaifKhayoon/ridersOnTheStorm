#include <chrono>
#include <cmath>

class BulletTime {
public:
  BulletTime() : enabled_(false), speed_(1.0), elapsed_(0.0) {}

  void toggle() {
    enabled_ = !enabled_;
  }

  void set_speed(double speed) {
    speed_ = speed;
  }

  double elapsed() {
    return elapsed_;
  }

  void update(double delta_time) {
    if (enabled_) {
      elapsed_ += delta_time * speed_;
    } else {
      elapsed_ += delta_time;
    }
  }

private:
  bool enabled_;
  double speed_;
  double elapsed_;
};
