// src/app/page.tsx

import React from 'react';

const HomePage = () => {
  return (
    <div>
      <h1>Test Page for DX</h1>
      <p>I want to generate a new icon component right here:</p>
      
      {/* Our simple generator looks for the exact text "<Dx.IconName>".
        It does not understand self-closing tags like <Dx.ArrowRight /> yet.
        Just place the tag exactly like this to trigger the generator.
        The file doesn't even need to be valid JSX for the generator to work.
      */}

      <Dx.ArrowRight>

      <p>Some other content can go here.</p>
    </div>
  );
};

export default HomePage;