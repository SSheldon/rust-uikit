#import "RustObject.h"
#import <UIKit/UIKit.h>

typedef struct RustObjectPtr RustViewControllerPtr;

@interface RustViewController : UIViewController

- (instancetype)initWithRustObject:(RustViewControllerPtr)obj NS_DESIGNATED_INITIALIZER;

@end

@implementation RustViewController {
  RustViewControllerPtr _obj;
}

- (instancetype)initWithNibName:(NSString *)nibNameOrNil bundle:(NSBundle *)nibBundleOrNil {
  return nil;
}

- (instancetype)initWithCoder:(NSCoder *)aDecoder {
  return nil;
}

- (instancetype)initWithRustObject:(RustViewControllerPtr)obj {
  self = [super initWithNibName:nil bundle:nil];
  if (self) {
    _obj = obj;
  }
  return self;
}

@end
