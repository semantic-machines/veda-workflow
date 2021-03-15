package com.semanticmachines.veda.bpm;

import org.sm.vqueue.VQueue;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.Properties;

public class VedaQueueWriter {
  
  private static VedaQueueWriter instance = null;
  public VQueue queue;
  private static final Properties properties = new Properties();
  private static String queue_path;

  protected VedaQueueWriter() {
    loadProperties();
    try {
      queue = new VQueue();
      queue.setNameAndPath("camunda-event", queue_path);
    } catch (Exception e) {
      e.printStackTrace();
    }
  }

  public static VedaQueueWriter getInstance() {
    if (instance == null) {
      instance = new VedaQueueWriter();
    }
    return instance;
  }

  private static void writeDefaultProperties() {
    System.out.println("Writing default queue properties");
    queue_path = "./data/queue/camunda-events";
    properties.setProperty("queue_path", queue_path);
    try {
      properties.store(new FileOutputStream("camunda-veda-plugin.properties"), null);
    } catch (IOException e) {
      e.printStackTrace();
    }
  }
  
  private static void loadProperties() {
    try {
      properties.load(new FileInputStream("camunda-veda-plugin.properties"));
      queue_path = properties.getProperty("queue_path", "");
      System.out.println(properties);
    } catch (IOException e) {
      writeDefaultProperties();
    }
  }
  
}
