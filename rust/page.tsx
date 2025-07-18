import React from 'react';

// This can be any component.
const MyComponent = () => {
  return (
    <div>
      <h1>Hello from DX!</h1>
      {/* This should now work perfectly! */}
      <Dx.ArrowRight />
    </div>
  );
};

export default MyComponent;