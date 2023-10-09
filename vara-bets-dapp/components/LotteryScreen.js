import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

const LotteryScreen = () => {
  return (
    <View style={styles.container}>
      <Text>Lottery Screen</Text>
      <Text>
      A lottery that supports millions or even tens of millions of people betting, similar to: EuroMillions lottery, Australia's Australian lottery. At the same time, some characteristics of vara chain are used to ensure the transparency and reliability of the lottery process.

      </Text>
    </View>
  );
};

export default LotteryScreen;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});