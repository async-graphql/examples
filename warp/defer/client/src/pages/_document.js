import React from 'react';
import Document from 'next/document';

export default class MyDocument extends Document {
  static async getInitialProps(ctx) {
    const initialProps = await Document.getInitialProps(ctx);

    const styles = [
      <style
        dangerouslySetInnerHTML={{
          __html: `
            #__next {
              height: 100%;
            }
          `,
        }}
      />,
    ];
    return { ...initialProps, styles: React.Children.toArray(styles) };
  }
}
